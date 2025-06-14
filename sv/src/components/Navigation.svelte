<script>
  import { currentRoute, navigateTo } from '../router.js';
  
  export let connected = false;
  
  function handleNavClick(e, route) {
    e.preventDefault();
    navigateTo(route);
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

<nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
  <div class="container mx-auto px-4">
    <div class="flex items-center justify-between h-16">
      <!-- Logo/Brand -->
      <div class="flex items-center">
        <a href="/" class="flex items-center space-x-2" on:click={(e) => handleNavClick(e, 'home')}>
          <div class="w-8 h-8 bg-gradient-to-r from-blue-500 to-purple-600 rounded flex items-center justify-center text-white font-bold">
            R
          </div>
          <span class="text-xl font-bold text-gray-800">RabbitMQ Demos</span>
        </a>
      </div>
      
      <!-- Navigation Links -->
      <div class="hidden lg:flex items-center space-x-1">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center space-x-2 px-3 py-2 rounded-lg text-sm font-medium transition-colors {
              activeRoute === item.id 
                ? 'bg-blue-100 text-blue-700' 
                : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
            }"
            on:click={(e) => handleNavClick(e, item.id)}
          >
            <span>{item.icon}</span>
            <span>{item.name}</span>
          </a>
        {/each}
      </div>
      
      <!-- Connection Status -->
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <div class="w-3 h-3 rounded-full {connected ? 'bg-green-500' : 'bg-red-500'}"></div>
          <span class="text-sm text-gray-600 hidden sm:block">
            {connected ? 'Connected' : 'Disconnected'}
          </span>
        </div>
        
        <!-- Mobile menu button -->
        <button 
          class="lg:hidden p-2 rounded-lg text-gray-600 hover:text-gray-900 hover:bg-gray-100"
          on:click={() => document.getElementById('mobile-menu').classList.toggle('hidden')}
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
          </svg>
        </button>
      </div>
    </div>
    
    <!-- Mobile menu -->
    <div id="mobile-menu" class="lg:hidden hidden border-t border-gray-200">
      <div class="py-2 space-y-1">
        {#each navItems as item}
          <a
            href={item.href}
            class="flex items-center space-x-3 px-4 py-3 text-sm font-medium transition-colors {
              activeRoute === item.id 
                ? 'bg-blue-100 text-blue-700' 
                : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
            }"
            on:click={(e) => {
              handleNavClick(e, item.id);
              document.getElementById('mobile-menu').classList.add('hidden');
            }}
          >
            <span class="text-lg">{item.icon}</span>
            <span>{item.name}</span>
          </a>
        {/each}
      </div>
    </div>
  </div>
</nav>