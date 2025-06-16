<script>
  import { createEventDispatcher } from 'svelte';
  import { currentRoute, navigateTo } from '../router.js';
  
  import Icon from '@iconify/svelte';
  
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
    { id: 'home', name: 'Home', href: '/', icon: 'ph:house-duotone' },
    { id: 'comparison', name: 'Traditional vs Broker', href: '/comparison', icon: 'ph:scales-duotone' },
    { id: 'logger', name: 'Message Logger', href: '/logger', icon: 'ph:mailbox-duotone' },
    { id: 'workers', name: 'Number Doubler', href: '/workers', icon: 'ph:calculator-duotone' },
    { id: 'game', name: 'Race to 100 Game', href: '/game', icon: 'ph:flag-checkered-duotone' },
    { id: 'rpc', name: 'Status Checker', href: '/rpc', icon: 'ph:arrows-clockwise-duotone' },
    { id: 'drawing', name: 'Collaborative Drawing', href: '/drawing', icon: 'ph:pencil-line-duotone' },
    { id: 'simulator', name: 'Flow Simulator', href: '/simulator', icon: 'ph:play-pause-duotone' },
    { id: 'newsimulations', name: 'New Simulations (D3)', href: '/newsimulations', icon: 'ph:chart-bar-duotone' }
  ];
  
  $: activeRoute = $currentRoute;
</script>

<nav class="bg-white border-r border-neutral-200 h-full w-60 flex flex-col">
  <!-- Navigation Links -->
  <div class="flex flex-col space-y-1.5 py-3 px-1.5 overflow-y-auto">
    {#each navItems as item}
      <a
        href={item.href}
        class="flex items-center space-x-3 px-1.5 py-2 rounded-md text-sm font-medium transition-colors {
          activeRoute === item.id 
            ? 'bg-blue-100 text-blue-700' 
            : 'text-neutral-800 hover:text-neutral-900 hover:bg-neutral-100'
        }"
        on:click={(e) => handleNavClick(e, item.id)}
      >
        <div class="h-[22px] w-[22px] flex items-center justify-center">
          <Icon icon={item.icon} width="22" height="22"/>
        </div>
        <span class="truncate">{item.name}</span>
      </a>
    {/each}
  </div>
</nav>