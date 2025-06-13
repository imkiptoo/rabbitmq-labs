<script>
  import MessageFlowSimulator from '../components/MessageFlowSimulator.svelte';
  
  export let ws;
  export let connected;
</script>

<div class="space-y-6">
  <div class="bg-indigo-50 p-4 rounded-lg">
    <h1 class="text-2xl font-semibold text-indigo-900 mb-2">Message Flow Simulator</h1>
    <p class="text-indigo-700">Visualize and understand how messages flow through different RabbitMQ patterns with animated diagrams.</p>
  </div>
  
  <div class="bg-white border rounded-lg p-4">
    <h2 class="text-lg font-semibold mb-3">Features</h2>
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 text-sm">
      <div class="bg-blue-50 p-3 rounded">
        <h3 class="font-medium text-blue-900">Visual Diagrams</h3>
        <p class="text-blue-700">See producers, queues, exchanges, and consumers</p>
      </div>
      <div class="bg-green-50 p-3 rounded">
        <h3 class="font-medium text-green-900">Live Animation</h3>
        <p class="text-green-700">Watch messages move through the system</p>
      </div>
      <div class="bg-yellow-50 p-3 rounded">
        <h3 class="font-medium text-yellow-900">Real-time Data</h3>
        <p class="text-yellow-700">Shows actual messages from other demos</p>
      </div>
      <div class="bg-purple-50 p-3 rounded">
        <h3 class="font-medium text-purple-900">Interactive</h3>
        <p class="text-purple-700">Simulate messages and control playback</p>
      </div>
    </div>
  </div>
  
  <MessageFlowSimulator {ws} {connected} />
</div>