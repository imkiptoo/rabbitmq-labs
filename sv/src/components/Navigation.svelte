<script>
  import { createEventDispatcher } from 'svelte';
  import { currentRoute, navigateTo } from '../router.js';
  
  export let connected = false;
  export let mobile = false;
  
  const dispatch = createEventDispatcher();
  
  function handleNavClick(e, route) {
    e.preventDefault();
    navigateTo(route);
    if (mobile) {
      dispatch('navigate');
    }
  }
  
  const navItems = [
    { id: 'home', name: 'Home', href: '/', icon: '🏠' },
    { id: 'comparison', name: 'Traditional vs Broker', href: '/comparison', icon: '⚖️' },
    { id: 'logger', name: 'Message Logger', href: '/logger', icon: '📨' },
    { id: 'workers', name: 'Number Doubler', href: '/workers', icon: '⚙️' },
    { id: 'game', name: 'Race to 100 Game', href: '/game', icon: '🏁' },
    { id: 'rpc', name: 'Status Checker', href: '/rpc', icon: '🔄' },
    { id: 'drawing', name: 'Collaborative Drawing', href: '/drawing', icon: '🎨' },
    { id: 'simulator', name: 'Flow Simulator', href: '/simulator', icon: '📊' }
  ];
  
  $: activeRoute = $currentRoute;
</script>

<nav class="bg-white border-r border-neutral-200 h-full w-60 flex flex-col">
  <!-- Navigation Links -->
  <div class="flex flex-col space-y-1.5 py-3 overflow-y-auto">
    {#each navItems as item}
      <a
        href={item.href}
        class="flex items-center space-x-3 px-3 py-1 text-sm font-medium transition-colors {
          activeRoute === item.id 
            ? 'bg-blue-100 text-blue-700' 
            : 'text-neutral-800 hover:text-neutral-900 hover:bg-neutral-100'
        }"
        on:click={(e) => handleNavClick(e, item.id)}
      >
        <span class="text-lg">{item.icon}</span>
        <span class="truncate">{item.name}</span>
      </a>
    {/each}
  </div>
</nav>