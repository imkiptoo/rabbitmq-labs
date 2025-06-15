<script>
  import { onMount, onDestroy } from "svelte";
  import { currentRoute, initRouter, navigateTo } from "./router.js";
  import Navigation from "./components/Navigation.svelte";

  // Import pages
  import HomePage from "./pages/HomePage.svelte";
  import LoggerPage from "./pages/LoggerPage.svelte";
  import WorkersPage from "./pages/WorkersPage.svelte";
  import GamePage from "./pages/GamePage.svelte";
  import RpcPage from "./pages/RpcPage.svelte";
  import DrawingPage from "./pages/DrawingPage.svelte";
  import SimulatorPage from "./pages/SimulatorPage.svelte";
  import ComparisonPage from "./pages/ComparisonPage.svelte";

  let ws = null;
  let connected = false;
  let destroyRouter;
  let mobileMenuOpen = false;

  const pages = {
    home: HomePage,
    logger: LoggerPage,
    workers: WorkersPage,
    game: GamePage,
    rpc: RpcPage,
    drawing: DrawingPage,
    simulator: SimulatorPage,
    comparison: ComparisonPage,
  };

  onMount(() => {
    destroyRouter = initRouter();
    connectWebSocket();
  });

  onDestroy(() => {
    if (destroyRouter) destroyRouter();
    if (ws) ws.close();
  });

  function connectWebSocket() {
    try {
      ws = new WebSocket("ws://localhost:3030/ws");

      ws.onopen = () => {
        connected = true;
        console.log("WebSocket connected");
      };

      ws.onclose = () => {
        connected = false;
        console.log("WebSocket disconnected");
        // Reconnect after 3 seconds
        setTimeout(connectWebSocket, 3000);
      };

      ws.onerror = (error) => {
        console.error("WebSocket error:", error);
      };

      ws.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          // Dispatch custom event for components to listen to
          window.dispatchEvent(
            new CustomEvent("websocket-message", { detail: message })
          );
        } catch (error) {
          console.error("Error parsing WebSocket message:", error);
        }
      };
    } catch (error) {
      console.error("Error connecting to WebSocket:", error);
      setTimeout(connectWebSocket, 3000);
    }
  }

  $: currentPage = pages[$currentRoute] || HomePage;
  $: pageNeedsWebSocket = $currentRoute !== "home";
</script>

<div class="h-screen w-full flex flex-col">
  <header
    class="flex h-11 items-center justify-between bg-black text-white shadow-md flex-shrink-0"
  >
    <div class="flex items-center space-x-3">
      <!-- Mobile menu button -->
      <button
        class="lg:hidden p-2 rounded-lg text-white hover:bg-neutral-700"
        on:click={() => (mobileMenuOpen = !mobileMenuOpen)}
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
        </svg>
      </button>

      <a
        href="/"
        class="flex items-center space-x-3"
        on:click={(e) => {
          e.preventDefault();
          navigateTo("home");
        }}
      >
        <img src="assets/rabbitmq-logo.svg" alt="Logo" class="w-6 h-6 mr-3" />
        <span class="font-bold text-white">RabbitMQ Demos</span>
      </a>
    </div>

    <!-- Connection Status in Header -->
    <div class="flex items-center space-x-3 px-3">
      <div
        class="w-3 h-3 rounded-full {connected ? 'bg-green-500' : 'bg-red-500'}"
      ></div>
      <span class="text-sm text-white hidden sm:block">
        {connected ? "Connected" : "Disconnected"}
      </span>
    </div>
  </header>

  <div class="flex flex-1 overflow-hidden relative">
    <!-- Desktop Sidebar -->
    <div class="hidden lg:block flex-shrink-0">
      <Navigation {connected} />
    </div>

    <!-- Mobile Navigation Overlay -->
    {#if mobileMenuOpen}
      <div class="lg:hidden fixed inset-0 z-50 flex">
        <!-- Backdrop -->
        <div
          class="fixed inset-0 bg-black bg-opacity-50"
          role="button"
          tabindex="-1"
          on:click={() => (mobileMenuOpen = false)}
          on:keydown={(e) => e.key === "Escape" && (mobileMenuOpen = false)}
        ></div>

        <!-- Mobile Menu -->
        <div class="relative bg-white w-64 h-full shadow-lg">
          <div class="flex items-center justify-between p-4 border-b">
            <span class="text-lg font-semibold text-neutral-800">Menu</span>
            <button
              class="p-2 rounded-lg text-neutral-600 hover:bg-neutral-100"
              on:click={() => (mobileMenuOpen = false)}
            >
              <svg
                class="w-5 h-5"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M6 18L18 6M6 6l12 12"
                ></path>
              </svg>
            </button>
          </div>
          <Navigation
            {connected}
            mobile={true}
            on:navigate={() => (mobileMenuOpen = false)}
          />
        </div>
      </div>
    {/if}

    <main class="flex-1 overflow-auto bg-neutral-100">
      <div class="container mx-auto px-3 py-3 max-w-none">
        {#if pageNeedsWebSocket}
          <svelte:component this={currentPage} {ws} {connected} />
        {:else}
          <svelte:component this={currentPage} />
        {/if}
      </div>
    </main>
  </div>
  <footer
    class="text-white text-center h-6 pb-0.5 flex items-center justify-between px-3 shadow {connected
      ? 'bg-black'
      : 'bg-red-700'}"
  >
    <div class="flex space-x-3 text-sm">
      <div class="">Built with ❤️ using <a href="https://svelte.dev" target="_blank" class="text-white hover:underline">Svelte</a></div>
    </div>
    <div class="flex space-x-2 items-center justify-center text-sm">
      <div>
        Status:
      </div>
      {#if  connected}
      <div>Connected</div>
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 16 16" {...$$props} class="text-green-500">
        <path fill="currentColor" d="M8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12M8 3c.374 0 .875.356 1.313 1.318q.141.313.26.682H6.427a6 6 0 0 1 .26-.682C7.125 3.356 7.627 3 8 3m-2.223.904q-.227.5-.393 1.096H4a5 5 0 0 1 2.038-1.6a6 6 0 0 0-.261.504M5.163 6A12 12 0 0 0 5 8c0 .699.057 1.373.163 2H3.416A5 5 0 0 1 3 8c0-.711.148-1.388.416-2zm.221 5q.166.596.393 1.096q.119.262.26.504A5 5 0 0 1 4 11zm1.043 0h3.146a6 6 0 0 1-.26.682C8.875 12.644 8.373 13 8 13c-.374 0-.875-.356-1.313-1.318a6 6 0 0 1-.26-.682m3.394-1H6.18A11 11 0 0 1 6 8c0-.714.064-1.39.179-2H9.82c.115.61.179 1.286.179 2s-.064 1.39-.179 2m.795 1H12a5 5 0 0 1-2.038 1.6q.143-.242.26-.504q.229-.5.394-1.096m1.968-1h-1.747A12 12 0 0 0 11 8c0-.699-.057-1.372-.163-2h1.747c.268.612.416 1.289.416 2s-.148 1.388-.416 2M9.962 3.4A5 5 0 0 1 12 5h-1.384a7.5 7.5 0 0 0-.393-1.096a6 6 0 0 0-.26-.504" />
      </svg>
      {:else}
      <div>Not Connected</div>
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 16 16" {...$$props} class="text-white">
        <path fill="currentColor" d="M2 8a6 6 0 0 0 4.509 5.813A5.5 5.5 0 0 1 6 11.5a5.5 5.5 0 0 1 .207-1.5h-.028A11 11 0 0 1 6 8c0-.714.064-1.39.179-2H9.82q.024.122.043.247a5.5 5.5 0 0 1 .98-.208L10.837 6h1.747l.05.117a5.5 5.5 0 0 1 1.18.392A6 6 0 0 0 2 8m6-5c.374 0 .875.356 1.313 1.318q.141.313.26.682H6.427a6 6 0 0 1 .26-.682C7.125 3.356 7.627 3 8 3m-2.223.904q-.227.5-.393 1.096H4a5 5 0 0 1 2.038-1.6a6 6 0 0 0-.261.504M5.163 6A12 12 0 0 0 5 8c0 .699.057 1.373.163 2H3.416A5 5 0 0 1 3 8c0-.711.148-1.388.416-2zm.221 5q.166.596.393 1.096q.119.262.26.504A5 5 0 0 1 4 11zm4.578-7.6A5 5 0 0 1 12 5h-1.384a7.5 7.5 0 0 0-.393-1.096a6 6 0 0 0-.26-.504M16 11.5a4.5 4.5 0 1 1-9 0a4.5 4.5 0 0 1 9 0M11.5 9a.5.5 0 0 0-.5.5v2a.5.5 0 0 0 1 0v-2a.5.5 0 0 0-.5-.5m0 5.125a.625.625 0 1 0 0-1.25a.625.625 0 0 0 0 1.25" />
      </svg>
      {/if}
    </div>
  </footer>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    overflow: hidden;
  }

  :global(#app) {
    height: 100vh;
  }
</style>
