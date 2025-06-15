<script>
  import TraditionalVsMessageBrokerComparison from '../components/TraditionalVsMessageBrokerComparison.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-gradient-to-r from-red-50 via-purple-50 to-blue-50 p-6 rounded-lg">
    <h1 class="text-3xl font-semibold text-neutral-900 mb-3">🔗 Traditional vs 🚀 Message Broker</h1>
    <p class="text-neutral-700 text-lg">See why smart companies are switching from direct service calls to message brokers!</p>
    <div class="mt-4 text-center">
      <div class="inline-flex items-center bg-yellow-100 border border-yellow-300 rounded-lg px-3 py-2">
        <span class="text-yellow-800 font-medium">👇 Try the scenarios below to see the difference!</span>
      </div>
    </div>
  </div>
  
  <div class="bg-white border rounded-lg p-6">
    <h2 class="text-xl font-semibold mb-4">😰 Why Traditional Way Causes Problems</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="space-y-4">
        <div class="bg-red-50 p-4 rounded-lg border-l-4 border-red-400">
          <h3 class="font-semibold text-red-800 mb-2">🔗 Everything Connected</h3>
          <p class="text-red-700 text-sm">Like dominoes - when one service calls another directly, they become tightly connected. Change one, break another!</p>
        </div>
        
        <div class="bg-orange-50 p-4 rounded-lg border-l-4 border-orange-400">
          <h3 class="font-semibold text-orange-800 mb-2">💥 One Breaks, All Break</h3>
          <p class="text-orange-700 text-sm">Just like a chain reaction - if the payment service goes down, your entire order system crashes!</p>
        </div>
      </div>
      
      <div class="space-y-4">
        <div class="bg-yellow-50 p-4 rounded-lg border-l-4 border-yellow-400">
          <h3 class="font-semibold text-yellow-800 mb-2">🐌 Traffic Jams</h3>
          <p class="text-yellow-700 text-sm">When lots of users come at once, services get stuck waiting for each other. Like rush hour traffic!</p>
        </div>
        
        <div class="bg-pink-50 p-4 rounded-lg border-l-4 border-pink-400">
          <h3 class="font-semibold text-pink-800 mb-2">😡 Angry Users</h3>
          <p class="text-pink-700 text-sm">Slow responses mean frustrated customers. Nobody wants to wait 30 seconds to place an order!</p>
        </div>
      </div>
    </div>
  </div>

  <div class="bg-white border rounded-lg p-6">
    <h2 class="text-xl font-semibold mb-4">🚀 How Message Brokers Save The Day!</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="space-y-4">
        <div class="bg-blue-50 p-4 rounded-lg border-l-4 border-blue-400">
          <h3 class="font-semibold text-blue-800 mb-2">📬 Smart Middleman</h3>
          <p class="text-blue-700 text-sm">Like having a super-smart postal service! Services don't talk directly - they send messages through the broker.</p>
        </div>
        
        <div class="bg-green-50 p-4 rounded-lg border-l-4 border-green-400">
          <h3 class="font-semibold text-green-800 mb-2">📦 Never Lose Messages</h3>
          <p class="text-green-700 text-sm">Even when services crash, messages wait safely in the queue. It's like having a super reliable mailbox!</p>
        </div>
      </div>
      
      <div class="space-y-4">
        <div class="bg-purple-50 p-4 rounded-lg border-l-4 border-purple-400">
          <h3 class="font-semibold text-purple-800 mb-2">🏎️ Handle Any Load</h3>
          <p class="text-purple-700 text-sm">Black Friday traffic? No problem! The queue manages everything smoothly, like a traffic controller.</p>
        </div>
        
        <div class="bg-indigo-50 p-4 rounded-lg border-l-4 border-indigo-400">
          <h3 class="font-semibold text-indigo-800 mb-2">😊 Happy Users</h3>
          <p class="text-indigo-700 text-sm">Fast responses even when things get crazy. Users get instant confirmations while work happens behind the scenes.</p>
        </div>
      </div>
    </div>
  </div>

  <TraditionalVsMessageBrokerComparison {ws} {connected} />
  
  <div class="bg-white border rounded-lg p-6">
    <h2 class="text-xl font-semibold mb-4">🎯 The Bottom Line</h2>
    <div class="space-y-3">
      <div class="flex items-start space-x-3">
        <div class="w-8 h-8 bg-green-500 text-white rounded-full flex items-center justify-center text-lg font-bold mt-0.5">💪</div>
        <p class="text-neutral-700"><strong>Stronger Systems:</strong> When one thing breaks, everything else keeps working!</p>
      </div>
      
      <div class="flex items-start space-x-3">
        <div class="w-8 h-8 bg-blue-500 text-white rounded-full flex items-center justify-center text-lg font-bold mt-0.5">🚀</div>
        <p class="text-neutral-700"><strong>Handle More Users:</strong> Ready for Black Friday, Super Bowl, or any traffic spike!</p>
      </div>
      
      <div class="flex items-start space-x-3">
        <div class="w-8 h-8 bg-purple-500 text-white rounded-full flex items-center justify-center text-lg font-bold mt-0.5">🔧</div>
        <p class="text-neutral-700"><strong>Easier to Fix:</strong> Update one service without breaking everything else!</p>
      </div>
      
      <div class="flex items-start space-x-3">
        <div class="w-8 h-8 bg-orange-500 text-white rounded-full flex items-center justify-center text-lg font-bold mt-0.5">⚡</div>
        <p class="text-neutral-700"><strong>Lightning Fast:</strong> Users get instant responses, even when your servers are busy!</p>
      </div>
    </div>
  </div>

  <div class="bg-gradient-to-r from-blue-50 to-green-50 p-6 rounded-lg">
    <h2 class="text-xl font-semibold text-neutral-900 mb-3">🏆 Why Companies Are Switching</h2>
    <p class="text-neutral-700 text-lg">
      Netflix, Uber, Amazon - they all use message brokers because they work better! 
      The simulation above shows exactly why: when things get tough, message brokers keep your business running 
      while traditional systems fall apart. 
    </p>
    <div class="mt-4 p-4 bg-yellow-100 border border-yellow-300 rounded-lg">
      <p class="text-yellow-800 font-medium text-center">
        🎉 Try clicking through the scenarios above - you'll see the difference immediately!
      </p>
    </div>
  </div>
</div>