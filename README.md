# RabbitMQ Demo Project

A unified project demonstrating 4 core RabbitMQ concepts with a Rust backend and Svelte frontend.

## Features

This project demonstrates the following RabbitMQ messaging patterns:

1. **Message Logger** - Basic queue (send/receive)
2. **Number Doubler Workers** - Work queues with multiple consumers
3. **Race to 100 Game** - Fanout exchange (pub/sub)
4. **Task Status Checker** - RPC request/reply pattern

## Prerequisites

- **RabbitMQ Server** - Install and run locally on default port (5672)
- **Rust** - Latest stable version
- **Node.js** - Version 16 or higher
- **npm** - For frontend dependencies

### Installing RabbitMQ

**On macOS (with Homebrew):**
```bash
brew install rabbitmq
brew services start rabbitmq
```

**On Ubuntu/Debian:**
```bash
sudo apt-get install rabbitmq-server
sudo systemctl start rabbitmq-server
```

**On Windows:**
Download and install from [RabbitMQ official website](https://www.rabbitmq.com/download.html)

## Quick Start

1. **Start RabbitMQ locally**
   ```bash
   # Make sure RabbitMQ is running on localhost:5672
   # Default guest/guest credentials are used
   ```

2. **Run the Rust backend:**
   ```bash
   cd rs
   cargo run
   ```
   The backend will start on `http://localhost:3030`

3. **Run the Svelte frontend:**
   ```bash
   cd sv
   npm install
   npm run dev
   ```
   The frontend will start on `http://localhost:5173`

4. **Open your browser**
   Navigate to `http://localhost:5173` to see the demo interface

## Project Structure

```
rabbitmq-demos/
├── rs/                     # Rust backend
│   ├── Cargo.toml         # Rust dependencies
│   ├── src/
│   │   ├── main.rs        # Main server with WebSocket & REST
│   │   ├── rabbitmq.rs    # RabbitMQ connection management
│   │   └── handlers/      # Demo-specific handlers
│   │       ├── mod.rs
│   │       ├── logger.rs  # Message logger demo
│   │       ├── workers.rs # Worker queue demo
│   │       ├── game.rs    # Race game demo
│   │       └── rpc.rs     # RPC demo
├── sv/                    # Svelte frontend
│   ├── package.json       # Node.js dependencies
│   ├── vite.config.js     # Vite configuration
│   ├── tailwind.config.js # Tailwind CSS config
│   ├── src/
│   │   ├── App.svelte     # Main app with tab navigation
│   │   ├── main.js        # Entry point
│   │   └── components/    # Demo components
│   │       ├── MessageLogger.svelte
│   │       ├── NumberDoubler.svelte
│   │       ├── RaceGame.svelte
│   │       └── StatusChecker.svelte
└── README.md
```

## Demo Details

### 1. Message Logger
- **Pattern:** Basic Queue
- **Usage:** Send text messages that are queued and displayed in real-time
- **RabbitMQ Concepts:** Simple producer/consumer, basic queue operations

### 2. Number Doubler Workers
- **Pattern:** Work Queues
- **Usage:** Submit numbers to be processed by multiple worker processes
- **RabbitMQ Concepts:** Work distribution, multiple consumers, message acknowledgment

### 3. Race to 100 Game
- **Pattern:** Fanout Exchange (Pub/Sub)
- **Usage:** Multiplayer clicking game with live leaderboard
- **RabbitMQ Concepts:** Broadcasting messages, multiple subscribers

### 4. Task Status Checker
- **Pattern:** RPC (Request/Reply)
- **Usage:** Check server status with request/response pattern
- **RabbitMQ Concepts:** Correlation IDs, reply queues, synchronous communication

## API Endpoints

The Rust backend provides the following REST endpoints:

- `POST /api/logger/send` - Send a message to the logger queue
- `POST /api/workers/submit` - Submit a number for worker processing
- `POST /api/game/click` - Register a click in the race game
- `GET /api/game/scores` - Get current game scores
- `POST /api/rpc/status` - Check server status via RPC
- `WS /ws` - WebSocket endpoint for real-time updates

## WebSocket Communication

All demos use WebSocket for real-time updates. Messages follow this format:

```json
{
  "demo_type": "logger|workers|game|rpc",
  "data": {
    // Demo-specific data
  }
}
```

## Technologies Used

### Backend (Rust)
- **lapin** - RabbitMQ client
- **tokio** - Async runtime
- **warp** - Web framework
- **serde** - Serialization
- **tokio-tungstenite** - WebSocket support

### Frontend (Svelte)
- **Svelte** - UI framework
- **Vite** - Build tool
- **Tailwind CSS** - Styling
- **WebSocket API** - Real-time communication

## Troubleshooting

### RabbitMQ Connection Issues
- Ensure RabbitMQ is running: `sudo systemctl status rabbitmq-server`
- Check if port 5672 is open and accessible
- Verify guest user has permissions (default setup)

### Backend Issues
- Check if port 3030 is available
- Look at console output for RabbitMQ connection errors
- Ensure all Rust dependencies are installed: `cargo check`

### Frontend Issues
- Ensure port 5173 is available
- Check browser console for WebSocket connection errors
- Verify npm dependencies are installed: `npm install`

### CORS Issues
- The backend is configured to allow all origins for development
- If you change ports, update the fetch URLs in Svelte components

## Development

### Adding New Demos
1. Create a new handler in `rs/src/handlers/`
2. Add the handler to `rs/src/handlers/mod.rs`
3. Add routes in `rs/src/main.rs`
4. Create a new Svelte component in `sv/src/components/`
5. Add the component to the tabs array in `App.svelte`

### Customization
- Modify RabbitMQ connection settings in `rs/src/rabbitmq.rs`
- Change styling by editing Tailwind classes
- Add new messaging patterns by extending the handlers

## License

This project is provided as-is for educational and demonstration purposes.