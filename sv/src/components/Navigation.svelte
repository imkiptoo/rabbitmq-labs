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
    { id: 'game', name: 'Race to 100', href: '/game', icon: '🏁' },
    { id: 'rpc', name: 'Status Checker', href: '/rpc', icon: '🔄' },
    { id: 'drawing', name: 'Collaborative Drawing', href: '/drawing', icon: '🎨' },
    { id: 'simulator', name: 'Flow Simulator', href: '/simulator', icon: '📊' }
  ];
  
  $: activeRoute = $currentRoute;
</script>

<nav class="bg-white border-r border-gray-200 h-full w-64 flex flex-col">
  <!-- Navigation Links -->
  <div class="flex flex-col py-4 px-2 space-y-1 overflow-y-auto">
    {#each navItems as item}
      <a
        href={item.href}
        class="flex items-center space-x-3 px-3 py-3 rounded-lg text-sm font-medium transition-colors {
          activeRoute === item.id 
            ? 'bg-blue-100 text-blue-700 border-r-2 border-blue-500' 
            : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
        }"
        on:click={(e) => handleNavClick(e, item.id)}
      >
        <span class="text-lg">{item.icon}</span>
        <span class="truncate">{item.name}</span>
      </a>
    {/each}
  </div>
</nav>