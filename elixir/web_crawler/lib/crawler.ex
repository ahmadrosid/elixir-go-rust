defmodule WebCrawler do
  def start(url) do
    total =
      Utils.measure(fn ->
        WebCrawler.run(url, fn doc ->
          doc
          |> Floki.find("a")
          |> Enum.map(&Floki.attribute(&1, "href"))
          |> List.flatten()
          |> Enum.filter(&(!String.starts_with?(&1, "#")))
        end)
        |> Enum.into([])
      end)
      |> Enum.reject(fn x -> x == "" end)
      |> length

    IO.puts("Total links scraped: #{total}")
  end

  def run(start_url, scraper_fun) do
    IO.puts("Start crawling for #{start_url}")

    Stream.resource(
      fn -> {[start_url], []} end,
      fn
        {[], _found_urls} ->
          {:halt, []}

        {urls, found_urls} ->
          {new_urls, data} = crawl(urls, scraper_fun)

          new_urls =
            new_urls
            |> List.flatten()
            |> Enum.uniq()
            |> Enum.reject(&diff_host?(URI.parse(start_url), &1))
            |> Enum.map(&to_string/1)
            |> Enum.reject(&Enum.member?(found_urls, &1))

          {data, {new_urls, found_urls ++ new_urls}}
      end,
      fn _ -> IO.puts("Finished crawling for '#{start_url}'.") end
    )
  end

  defp crawl(urls, scraper_fun) when is_list(urls) do
    IO.puts("Start async crawler!")

    urls
    |> Task.async_stream(&crawl(&1, scraper_fun),
      ordered: false,
      timeout: 15_000 * 100,
      max_concurrency: 100
    )
    |> Enum.into([], fn {_key, value} -> value end)
    # |> Enum.map(&crawl(&1, scraper_fun))
    |> Enum.reduce({[], []}, fn {scraped_urls, scraped_data}, {acc_urls, acc_data} ->
      {scraped_urls ++ acc_urls, scraped_data ++ acc_data}
    end)
  end

  defp crawl(url, scraper_fun) when is_binary(url) do
    IO.puts(IO.ANSI.green() <> "#{inspect(self())} Scraping url: #{url}" <> IO.ANSI.reset())

    user_agent_headers = [
      {"User-Agent",
       "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"}
    ]

    with {:http, {:ok, %HTTPoison.Response{status_code: 200, body: body}}} <-
           {:http,
            HTTPoison.get(url, user_agent_headers,
              timeout: 15_000,
              recv_timeout: 15_000,
              follow_redirect: true,
              hackney: [pool: :default]
            )},
         {:parse, {:ok, parsed_document}} <- {:parse, Floki.parse_document(body)} do
      new_urls =
        parsed_document
        |> Floki.find("a[href]")
        |> Floki.attribute("href")
        |> Enum.map(&URI.merge(url, &1))
        |> Enum.map(&Utils.clean_url(&1))

      new_data =
        case scraper_fun.(parsed_document) do
          x when is_list(x) -> x
          x -> [x]
        end

      {new_urls, [] ++ new_data}
    else
      {:http, {:ok, %HTTPoison.Response{}}} ->
        IO.puts(
          IO.ANSI.yellow() <> "Ignore, failed to fetch html content #{url}" <> IO.ANSI.reset()
        )

        {[], []}

      {:http, {:error, %HTTPoison.Error{reason: reason}}} ->
        IO.puts(IO.ANSI.yellow() <> "Failed to get #{url}, because: #{reason}" <> IO.ANSI.reset())
        {[], []}

      {:parse, {:error, error}} ->
        IO.puts(
          IO.ANSI.yellow() <> "Failed to parse #{url}, because: #{error}" <> IO.ANSI.reset()
        )

        {[], []}
    end
  end

  defp diff_host?(%URI{host: first_host}, %URI{host: second_host}),
    do: first_host != second_host
end
