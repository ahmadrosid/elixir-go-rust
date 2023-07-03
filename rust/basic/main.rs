use tokio::sync::mpsc;
use tokio::task;

async fn process_messages(messages: Vec<String>) {
    for message in messages {
        println!("Processing message: {}", message);
        // Simulating some work
        // You can replace this with actual processing logic
        // e.g., calling a function, making an API request, etc.
    }
}

#[tokio::main]
async fn main() {
    let messages = vec![
        "Message 1".to_string(),
        "Message 2".to_string(),
        "Message 3".to_string(),
        "Message 4".to_string(),
        "Message 5".to_string(),
    ];

    let (tx, mut rx) = mpsc::channel::<Vec<String>>(messages.len());

    tokio::spawn(async move {
        for message in messages {
            let tx = tx.clone();
            tokio::spawn(async move {
                process_messages(vec![message]).await;
                tx.send(vec![]).await.expect("Failed to send completion signal");
            });
        }
    });

    while let Some(completed_task) = rx.recv().await {
        if completed_task.is_empty() {
            break;
        }
    }

    println!("All messages processed!");
}
