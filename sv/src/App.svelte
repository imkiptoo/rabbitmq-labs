<script>
  import { onMount, onDestroy } from 'svelte';
  import { currentRoute, initRouter, navigateTo } from './router.js';
  import Navigation from './components/Navigation.svelte';
  
  // Import pages
  import HomePage from './pages/HomePage.svelte';
  import LoggerPage from './pages/LoggerPage.svelte';
  import WorkersPage from './pages/WorkersPage.svelte';
  import GamePage from './pages/GamePage.svelte';
  import RpcPage from './pages/RpcPage.svelte';
  import DrawingPage from './pages/DrawingPage.svelte';
  import SimulatorPage from './pages/SimulatorPage.svelte';
  import ComparisonPage from './pages/ComparisonPage.svelte';

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
    comparison: ComparisonPage
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
      ws = new WebSocket('ws://localhost:3030/ws');
      
      ws.onopen = () => {
        connected = true;
        console.log('WebSocket connected');
      };
      
      ws.onclose = () => {
        connected = false;
        console.log('WebSocket disconnected');
        // Reconnect after 3 seconds
        setTimeout(connectWebSocket, 3000);
      };
      
      ws.onerror = (error) => {
        console.error('WebSocket error:', error);
      };
      
      ws.onmessage = (event) => {
        try {
          const message = JSON.parse(event.data);
          // Dispatch custom event for components to listen to
          window.dispatchEvent(new CustomEvent('websocket-message', { detail: message }));
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
        }
      };
    } catch (error) {
      console.error('Error connecting to WebSocket:', error);
      setTimeout(connectWebSocket, 3000);
    }
  }

  $: currentPage = pages[$currentRoute] || HomePage;
  $: pageNeedsWebSocket = $currentRoute !== 'home';
</script>


<div class="h-screen w-full flex flex-col">
  <header class="flex h-16 items-center justify-between bg-black px-4 text-white shadow-md flex-shrink-0">
    <div class="flex items-center space-x-4">
      <!-- Mobile menu button -->
      <button 
        class="lg:hidden p-2 rounded-lg text-white hover:bg-gray-700"
        on:click={() => mobileMenuOpen = !mobileMenuOpen}
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
        </svg>
      </button>
      
      <a href="/" class="flex items-center space-x-2" on:click={(e) => {e.preventDefault(); navigateTo('home');}}>
        <div class="w-8 h-8 bg-gradient-to-r from-blue-500 to-purple-600 rounded flex items-center justify-center text-white font-bold">
          R
        </div>
        <span class="font-bold text-white">RabbitMQ Demos</span>
      </a>
    </div>
    
    <!-- Connection Status in Header -->
    <div class="flex items-center space-x-2">
      <div class="w-3 h-3 rounded-full {connected ? 'bg-green-500' : 'bg-red-500'}"></div>
      <span class="text-sm text-white hidden sm:block">
        {connected ? 'Connected' : 'Disconnected'}
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
        <div class="fixed inset-0 bg-black bg-opacity-50" role="button" tabindex="-1" on:click={() => mobileMenuOpen = false} on:keydown={(e) => e.key === 'Escape' && (mobileMenuOpen = false)}></div>
        
        <!-- Mobile Menu -->
        <div class="relative bg-white w-64 h-full shadow-lg">
          <div class="flex items-center justify-between p-4 border-b">
            <span class="text-lg font-semibold text-gray-800">Menu</span>
            <button 
              class="p-2 rounded-lg text-gray-600 hover:bg-gray-100"
              on:click={() => mobileMenuOpen = false}
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          </div>
          <Navigation {connected} mobile={true} on:navigate={() => mobileMenuOpen = false} />
        </div>
      </div>
    {/if}
    
    <main class="flex-1 overflow-auto bg-gray-100">
      <div class="container mx-auto px-4 py-8 max-w-none">
        {#if pageNeedsWebSocket}
          <svelte:component this={currentPage} {ws} {connected} />
        {:else}
          <svelte:component this={currentPage} />
        {/if}
      </div>
    </main>
  </div>
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