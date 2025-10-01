# Elm Architecture in DroidTUI

## Overview

DroidTUI follows the **Elm Architecture** pattern, which provides a clean, functional, and maintainable way to structure applications. This architecture makes the code easier to understand, test, and extend.

## Core Concepts

The Elm Architecture consists of three main components:

```
┌─────────────────────────────────────────────┐
│                   MODEL                     │
│  (Application State - What we know)         │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│                   VIEW                      │
│  (Rendering - What the user sees)           │
└─────────────────┬───────────────────────────┘
                  │
                  ↓ (User events)
┌─────────────────────────────────────────────┐
│                 MESSAGE                     │
│  (Events - What can happen)                 │
└─────────────────┬───────────────────────────┘
                  │
                  ↓
┌─────────────────────────────────────────────┐
│                 UPDATE                      │
│  (State transitions - How state changes)    │
└─────────────────┬───────────────────────────┘
                  │
                  ↓ (New state)
              Back to MODEL
```

## File Structure

```
src/
├── app.rs        - Main application loop (connects everything)
├── model.rs      - Application state (Model)
├── message.rs    - All possible events (Message)
├── update.rs     - State transition logic (Update)
├── view.rs       - UI rendering (View)
├── menu.rs       - Menu component
├── effects.rs    - Visual effects
└── event.rs      - Event handling
```

## Component Details

### 1. Model (`model.rs`)

**Purpose**: Contains ALL application state in one place.

**Key Principles**:
- Immutable state (or mutated only through Update)
- Single source of truth
- No business logic

**Example**:
```rust
pub struct Model {
    pub state: AppState,
    pub menu: Menu,
    pub effects: EffectsManager,
    pub command_result: Option<String>,
    pub scroll_position: usize,
    // ... all state here
}
```

**Benefits**:
- Easy to understand what data the app holds
- Easy to serialize/deserialize
- No hidden state
- Predictable

### 2. Message (`message.rs`)

**Purpose**: Defines ALL possible actions/events that can occur.

**Key Principles**:
- Enum of all possible messages
- Descriptive names
- Carries data if needed

**Example**:
```rust
pub enum Message {
    MenuUp,
    MenuDown,
    ExecuteCommand(String),
    CommandCompleted(CommandResult),
    ScrollUp,
    Quit,
    // ... all possible actions
}
```

**Benefits**:
- Clear documentation of what can happen
- Type-safe event handling
- Easy to add new features
- Testable

### 3. Update (`update.rs`)

**Purpose**: Pure function that handles state transitions.

**Key Principles**:
- Takes current model + message → returns updated model
- Pure function (no side effects when possible)
- Single place for all state changes

**Example**:
```rust
pub async fn update(model: &mut Model, message: Message) {
    match message {
        Message::MenuUp => {
            model.menu.previous();
        }
        Message::Quit => {
            model.running = false;
        }
        // ... handle all messages
    }
}
```

**Benefits**:
- All state changes in one place
- Easy to understand how state changes
- Easy to test
- No scattered mutations

### 4. View (`view.rs`)

**Purpose**: Renders UI based on current model state.

**Key Principles**:
- Pure function (Model → UI)
- No state modifications
- No business logic

**Example**:
```rust
pub fn render(model: &mut Model, area: Rect, buf: &mut Buffer) {
    match model.state {
        AppState::Menu => render_menu(model, area, buf),
        AppState::ShowResult => render_result(model, area, buf),
        // ... render based on state
    }
}
```

**Benefits**:
- Predictable rendering
- Easy to modify UI
- No side effects
- Testable

### 5. App (`app.rs`)

**Purpose**: Main loop that connects Model-Update-View.

**The Loop**:
```rust
loop {
    // 1. VIEW: Render current state
    terminal.draw(|frame| render(&model, frame.area(), frame.buffer_mut()))?;
    
    // 2. EVENT: Wait for user input
    let event = events.next().await?;
    
    // 3. MESSAGE: Convert event to message
    let message = event_to_message(event)?;
    
    // 4. UPDATE: Update model with message
    update(&mut model, message).await;
    
    // 5. Repeat...
}
```

## Data Flow

```
User Input → Event → Message → Update → Model → View → Screen
     ↑                                                      ↓
     └──────────────────────────────────────────────────────┘
```

1. **User Input**: User presses a key
2. **Event**: Raw keyboard event received
3. **Message**: Event converted to semantic message (e.g., `MenuUp`)
4. **Update**: Message processed, model updated
5. **Model**: New state stored
6. **View**: UI rendered from new state
7. **Screen**: User sees result
8. Loop continues...

## Benefits of Elm Architecture

### 1. **Predictability**
- State changes only happen in `update()`
- Rendering only depends on current model
- No hidden state or side effects

### 2. **Testability**
```rust
#[test]
fn test_menu_navigation() {
    let mut model = Model::new();
    update(&mut model, Message::MenuDown).await;
    assert_eq!(model.menu.selected, 1);
}
```

### 3. **Maintainability**
- Clear separation of concerns
- Easy to find where things happen
- Easy to add new features

### 4. **Debuggability**
- All state in one place (Model)
- All actions listed (Message)
- All transitions in one place (Update)

### 5. **Scalability**
- Easy to add new states
- Easy to add new messages
- Easy to add new views

## Example: Adding a New Feature

Let's say we want to add a "Command History" feature:

### Step 1: Update Model
```rust
pub struct Model {
    // ... existing fields
    pub command_history: Vec<String>,
}
```

### Step 2: Add Messages
```rust
pub enum Message {
    // ... existing messages
    ShowHistory,
    ClearHistory,
}
```

### Step 3: Update Handler
```rust
pub async fn update(model: &mut Model, message: Message) {
    match message {
        // ... existing handlers
        Message::ShowHistory => {
            model.state = AppState::ShowHistory;
        }
        Message::ClearHistory => {
            model.command_history.clear();
        }
    }
}
```

### Step 4: Add View
```rust
pub fn render(model: &mut Model, area: Rect, buf: &mut Buffer) {
    match model.state {
        // ... existing states
        AppState::ShowHistory => render_history(model, area, buf),
    }
}
```

### Step 5: Wire Up Events
```rust
fn key_to_message(&self, key: KeyCode) -> Option<Message> {
    match key {
        // ... existing keys
        KeyCode::Char('h') => Some(Message::ShowHistory),
        // ...
    }
}
```

Done! The feature is complete and follows the architecture.

## Common Patterns

### Pattern 1: Async Operations
```rust
Message::ExecuteCommand(command) => {
    model.state = AppState::Loading;
    let result = execute_command(&command).await;
    model.set_result(result);
    model.state = AppState::ShowResult;
}
```

### Pattern 2: Conditional State Transitions
```rust
Message::EnterChild => {
    if model.menu.has_children() {
        model.menu.enter_child_mode();
        model.effects.start_fade_in();
    }
}
```

### Pattern 3: Derived State
```rust
impl Model {
    pub fn can_scroll(&self) -> bool {
        self.wrapped_lines.len() > self.visible_lines
    }
}
```

## Testing

The Elm Architecture makes testing straightforward:

### Unit Tests
```rust
#[test]
fn test_scroll_boundaries() {
    let mut model = Model::new();
    model.wrapped_lines = vec!["Line 1".into(), "Line 2".into()];
    
    update(&mut model, Message::ScrollDown).await;
    assert_eq!(model.scroll_position, 1);
}
```

### Integration Tests
```rust
#[test]
fn test_command_execution_flow() {
    let mut model = Model::new();
    
    // Execute command
    update(&mut model, Message::ExecuteCommand("echo test".into())).await;
    assert_eq!(model.state, AppState::ShowResult);
    assert!(model.command_result.is_some());
}
```

## Best Practices

### DO:
✅ Keep Model as simple data
✅ Make Messages descriptive
✅ Keep Update pure when possible
✅ Keep View as rendering only
✅ Handle all errors explicitly
✅ Write tests for Update logic

### DON'T:
❌ Mutate state outside Update
❌ Add business logic to View
❌ Create hidden state
❌ Skip messages for "convenience"
❌ Mix concerns across boundaries

## Comparison to Traditional Architecture

### Traditional (Object-Oriented)
```rust
struct App {
    menu: Menu,
    state: State,
}

impl App {
    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Up => {
                self.menu.previous();
                self.state = State::Menu;
                // State scattered everywhere
            }
        }
    }
}
```

### Elm Architecture
```rust
// State in one place
struct Model { ... }

// Actions in one place
enum Message { MenuUp, ... }

// Transitions in one place
fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::MenuUp => model.menu.previous(),
    }
}
```

## Further Reading

- [The Elm Architecture](https://guide.elm-lang.org/architecture/)
- [Redux (similar pattern)](https://redux.js.org/)
- [Command Pattern](https://en.wikipedia.org/wiki/Command_pattern)

## Summary

The Elm Architecture provides:
- **Clear structure**: Model, Message, Update, View
- **Predictable behavior**: All state changes in one place
- **Easy testing**: Pure functions are easy to test
- **Maintainable code**: Easy to understand and modify
- **Scalable design**: Easy to add new features

This architecture makes DroidTUI more maintainable, testable, and easier to understand!