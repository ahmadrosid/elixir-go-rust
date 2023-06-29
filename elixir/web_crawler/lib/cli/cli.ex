defmodule WebCrawler.CLI do
  def main(args) do
    args
    |> parse_args()
    |> execute()
  end

  defp parse_args(args) do
    args
    |> OptionParser.parse(strict: [site: :string, worker: :string, help: :boolean])
    |> elem(0)
  end

  def execute(help: true) do
    IO.puts("""
    usage: web_crawler [--help] [--site=<url>] [--worker=<int>]

    The most common use case is:
      $ web_crawler --site https://react.dev/ --worker 5

    It will crawl the website fetching all the urls within the same
    domain and save them to a text file in the same directory where
    the application has been executed.
    """)
  end

  def execute(site: site) do
    WebCrawler.start(site, 10)
  end

  def execute(site: site, worker: worker) do
    WebCrawler.start(site, String.to_integer(worker))
  end

  def execute(_) do
    IO.puts("That is not a valid web_crawler command. See 'web_crawler --help'.")
  end
end
