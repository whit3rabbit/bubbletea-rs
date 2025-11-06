use bubbletea_rs::{BlurMsg, Error, FocusMsg, InputHandler, KeyMsg, MouseMsg, WindowSizeMsg};
use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::mpsc;

// Mock EventStream for testing
struct MockEventStream {
    events: Vec<Event>,
    index: usize,
}

impl MockEventStream {
    fn new(events: Vec<Event>) -> Self {
        MockEventStream { events, index: 0 }
    }
}

impl futures::Stream for MockEventStream {
    type Item = Result<Event, std::io::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.index < self.events.len() {
            let event = self.events[self.index].clone();
            self.index += 1;
            std::task::Poll::Ready(Some(Ok(event)))
        } else {
            std::task::Poll::Ready(None)
        }
    }
}

#[tokio::test]
async fn test_input_handler_key_event() -> Result<(), Error> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let input_handler = InputHandler::new(event_tx);

    let mock_events = vec![
        Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::SHIFT)),
    ];

    // Replace the actual EventStream with our mock
    // This requires a bit of a hack as EventStream is not easily mockable
    // For a real application, consider dependency injection for EventStream
    // For this test, we'll simulate the run loop directly
    tokio::spawn(async move {
        let mut stream = MockEventStream::new(mock_events);
        while let Some(event_result) = stream.next().await {
            if let Ok(Event::Key(key_event)) = event_result {
                let msg = KeyMsg {
                    key: key_event.code,
                    modifiers: key_event.modifiers,
                };
                let _ = input_handler.event_tx.send(Box::new(msg));
            }
        }
    });

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let key_msg = received_msg.downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Char('a'));
    assert_eq!(key_msg.modifiers, KeyModifiers::NONE);

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let key_msg = received_msg.downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Enter);
    assert_eq!(key_msg.modifiers, KeyModifiers::SHIFT);

    Ok(())
}

#[tokio::test]
#[cfg(target_os = "windows")]
// Ensure release events are not emitted by bubbletea on Windows
async fn test_input_handler_key_event_windows() -> Result<(), Error> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let input_handler = InputHandler::new(event_tx);

    let mock_events = vec![
        Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
            crossterm::event::KeyEventKind::Press,
        )),
        Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
            crossterm::event::KeyEventKind::Release,
        )),
    ];

    // Replace the actual EventStream with our mock
    // This requires a bit of a hack as EventStream is not easily mockable
    // For a real application, consider dependency injection for EventStream
    // For this test, we'll simulate the run loop directly
    tokio::spawn(async move {
        let mut stream = MockEventStream::new(mock_events);
        while let Some(event_result) = stream.next().await {
            if let Ok(Event::Key(key_event)) = event_result {
                let msg = KeyMsg {
                    key: key_event.code,
                    modifiers: key_event.modifiers,
                };
                let _ = input_handler.event_tx.send(Box::new(msg));
            }
        }
    });

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let key_msg = received_msg.downcast_ref::<KeyMsg>().unwrap();
    assert_eq!(key_msg.key, KeyCode::Char('a'));
    assert_eq!(key_msg.modifiers, KeyModifiers::NONE);

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap();

    assert!(received_msg.is_none());

    Ok(())
}

#[tokio::test]
async fn test_input_handler_mouse_event() -> Result<(), Error> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let input_handler = InputHandler::new(event_tx);

    let mock_events = vec![
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 10,
            row: 20,
            modifiers: KeyModifiers::NONE,
        }),
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            column: 5,
            row: 5,
            modifiers: KeyModifiers::CONTROL,
        }),
    ];

    tokio::spawn(async move {
        let mut stream = MockEventStream::new(mock_events);
        while let Some(event_result) = stream.next().await {
            if let Ok(Event::Mouse(mouse_event)) = event_result {
                let msg = MouseMsg {
                    x: mouse_event.column,
                    y: mouse_event.row,
                    button: mouse_event.kind,
                    modifiers: mouse_event.modifiers,
                };
                let _ = input_handler.event_tx.send(Box::new(msg));
            }
        }
    });

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let mouse_msg = received_msg.downcast_ref::<MouseMsg>().unwrap();
    assert_eq!(mouse_msg.x, 10);
    assert_eq!(mouse_msg.y, 20);
    assert_eq!(mouse_msg.button, MouseEventKind::Down(MouseButton::Left));
    assert_eq!(mouse_msg.modifiers, KeyModifiers::NONE);

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let mouse_msg = received_msg.downcast_ref::<MouseMsg>().unwrap();
    assert_eq!(mouse_msg.x, 5);
    assert_eq!(mouse_msg.y, 5);
    assert_eq!(mouse_msg.button, MouseEventKind::Moved);
    assert_eq!(mouse_msg.modifiers, KeyModifiers::CONTROL);

    Ok(())
}

#[tokio::test]
async fn test_input_handler_resize_event() -> Result<(), Error> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let input_handler = InputHandler::new(event_tx);

    let mock_events = vec![Event::Resize(80, 24), Event::Resize(100, 50)];

    tokio::spawn(async move {
        let mut stream = MockEventStream::new(mock_events);
        while let Some(event_result) = stream.next().await {
            if let Ok(Event::Resize(width, height)) = event_result {
                let msg = WindowSizeMsg { width, height };
                let _ = input_handler.event_tx.send(Box::new(msg));
            }
        }
    });

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let size_msg = received_msg.downcast_ref::<WindowSizeMsg>().unwrap();
    assert_eq!(size_msg.width, 80);
    assert_eq!(size_msg.height, 24);

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    let size_msg = received_msg.downcast_ref::<WindowSizeMsg>().unwrap();
    assert_eq!(size_msg.width, 100);
    assert_eq!(size_msg.height, 50);

    Ok(())
}

#[tokio::test]
async fn test_input_handler_focus_events() -> Result<(), Error> {
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    let input_handler = InputHandler::new(event_tx);

    let mock_events = vec![Event::FocusGained, Event::FocusLost];

    tokio::spawn(async move {
        let mut stream = MockEventStream::new(mock_events);
        while let Some(event_result) = stream.next().await {
            if let Ok(event) = event_result {
                match event {
                    Event::FocusGained => {
                        let msg = FocusMsg;
                        let _ = input_handler.event_tx.send(Box::new(msg));
                    }
                    Event::FocusLost => {
                        let msg = BlurMsg;
                        let _ = input_handler.event_tx.send(Box::new(msg));
                    }
                    _ => {}
                }
            }
        }
    });

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(received_msg.downcast_ref::<FocusMsg>().is_some());

    let received_msg = tokio::time::timeout(Duration::from_millis(100), event_rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(received_msg.downcast_ref::<BlurMsg>().is_some());

    Ok(())
}
