<script>
  import { onMount, onDestroy } from 'svelte';
  import { currentRoute, initRouter } from './router.js';
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

<div class="min-h-screen bg-gray-100">
  <Navigation {connected} />
  
  <main class="container mx-auto px-4 py-8">
    {#if pageNeedsWebSocket}
      <svelte:component this={currentPage} {ws} {connected} />
    {:else}
      <svelte:component this={currentPage} />
    {/if}
  </main>
</div>