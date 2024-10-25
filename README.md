# HermesOS: A Minimal Web-Based Operating System

HermesOS is an experimental project aiming to create a minimal yet functional operating system that runs entirely within a web browser. The core idea is to provide a streamlined and efficient user experience by abstracting away the complexities of a traditional OS and focusing on running web applications.

## Architecture

HermesOS employs a hybrid architecture, combining a lightweight Rust/WebAssembly kernel with web applications that communicate with backend services via API calls. This approach minimizes client-side processing, leading to improved performance and reduced memory consumption.

**Key Components:**

* **Kernel (Rust/WebAssembly):** Handles core system functionalities such as window management, a virtual file system, and system calls.  This component is crucial for performance and leverages Rust's efficiency and WebAssembly's near-native speed in the browser.
* **Frontend (SvelteKit, Sass):** Provides the graphical user interface, window management, and interaction with the kernel. SvelteKit offers a modern and performant framework for building the UI. Sass helps manage styling efficiently.
* **Backend (Go with Gin):**  Handles business logic and data processing for the applications. Go's performance and Gin's minimalist approach make this a robust and efficient backend solution.
* **Database:** IndexedDB (local storage) and a cloud-based NoSQL database (to be defined). This combination allows for efficient data management both on the client-side and for persistent storage.
* **Authentication:** OAuth 2.0 (Google). Secure and widely adopted authentication method.

## Technologies Used

* **Frontend:** SvelteKit, pnpm, Sass, TypeScript, Prettier, ESLint, Vitest.
* **Backend:** Go, Gin.
* **Kernel:** Rust, WebAssembly, wasm-bindgen.
* **Database:** IndexedDB, PostgreSQL
* **Authentication:** OAuth 2.0 (Google).

## Getting Started

**Prerequisites:**

* Pnpm: `npm install -g pnpm`
* Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* wasm-pack: `cargo install wasm-pack`
* Go: [https://go.dev/doc/install](https://go.dev/doc/install)
* Node.js and npm (for some frontend dependencies): [https://nodejs.org/en/download](https://nodejs.org/en/download)

**Frontend:**

1. Navigate to the `frontend` directory: `cd frontend`
2. Install dependencies: `pnpm install`
3. Start the development server: `pnpm run dev`

**Backend:**

1. Navigate to the `backend` directory: `cd backend`
2. Install dependencies: `go mod tidy`
3. Run the API: `go run main.go`

**Kernel:**

1. Navigate to the `kernel` directory: `cd kernel`
2. Build for WebAssembly: `wasm-pack build --target web`
3. Copy `kernel.wasm` and `kernel_bg.wasm` (and any other generated files like `kernel.js`) from `kernel/pkg` to `frontend/static`.

## Build Process and Workflow

The kernel, written in Rust, is compiled to WebAssembly using `wasm-pack`. This process generates the necessary files for integration with the JavaScript frontend.  Changes to the Rust code require recompilation and copying the updated Wasm files to the frontend's static directory. A build script can automate this process (incoming).

The frontend, built with SvelteKit, uses pnpm for development and building.  The backend, written in Go, is built and run using standard Go tooling.

## Roadmap

* Implement core window management functionalities in the Rust kernel.
* Develop basic applications (calculator, text editor, file explorer).
* Integrate OAuth 2.0 authentication with Google.
* Implement the virtual file system using IndexedDB.
* Optimize the performance of the WebAssembly kernel.
* Design and implement inter-process communication.

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.  Please follow the contribution guidelines (CONTRIBUTING.md - to be created).

## License

[MIT](LICENSE)
