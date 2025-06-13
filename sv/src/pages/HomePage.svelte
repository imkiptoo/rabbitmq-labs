<div class="space-y-6">
  <div class="bg-gradient-to-r from-blue-500 to-purple-600 text-white p-8 rounded-lg">
    <h1 class="text-4xl font-bold mb-4">RabbitMQ Demos</h1>
    <p class="text-xl mb-6">Interactive demonstrations of RabbitMQ messaging patterns</p>
    <p class="text-blue-100">
      Explore different messaging patterns and see how messages flow through queues, exchanges, and consumers in real-time.
    </p>
  </div>
  
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
    <a href="/logger" class="block">
      <div class="bg-white border rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 bg-blue-500 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            📨
          </div>
          <h3 class="text-xl font-semibold ml-4">Message Logger</h3>
        </div>
        <p class="text-gray-600 mb-4">Simple producer-consumer pattern with basic message queuing.</p>
        <div class="text-sm text-blue-600 font-medium">Basic Queue Pattern →</div>
      </div>
    </a>
    
    <a href="/workers" class="block">
      <div class="bg-white border rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 bg-green-500 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            ⚙️
          </div>
          <h3 class="text-xl font-semibold ml-4">Number Doubler</h3>
        </div>
        <p class="text-gray-600 mb-4">Work queue with multiple competing consumers for load balancing.</p>
        <div class="text-sm text-green-600 font-medium">Work Queue Pattern →</div>
      </div>
    </a>
    
    <a href="/game" class="block">
      <div class="bg-white border rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 bg-yellow-500 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            🏁
          </div>
          <h3 class="text-xl font-semibold ml-4">Race to 100</h3>
        </div>
        <p class="text-gray-600 mb-4">Fanout exchange broadcasting score updates to all players.</p>
        <div class="text-sm text-yellow-600 font-medium">Pub/Sub Pattern →</div>
      </div>
    </a>
    
    <a href="/rpc" class="block">
      <div class="bg-white border rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 bg-red-500 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            🔄
          </div>
          <h3 class="text-xl font-semibold ml-4">Status Checker</h3>
        </div>
        <p class="text-gray-600 mb-4">Request-reply pattern with correlation IDs for RPC-style communication.</p>
        <div class="text-sm text-red-600 font-medium">RPC Pattern →</div>
      </div>
    </a>
    
    <a href="/drawing" class="block">
      <div class="bg-white border rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 bg-purple-500 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            🎨
          </div>
          <h3 class="text-xl font-semibold ml-4">Drawing Board</h3>
        </div>
        <p class="text-gray-600 mb-4">Collaborative real-time drawing with fanout broadcasting.</p>
        <div class="text-sm text-purple-600 font-medium">Collaboration Pattern →</div>
      </div>
    </a>
    
    <a href="/simulator" class="block">
      <div class="bg-white border rounded-lg p-6 hover:shadow-lg transition-shadow cursor-pointer">
        <div class="flex items-center mb-4">
          <div class="w-12 h-12 bg-indigo-500 rounded-lg flex items-center justify-center text-white font-bold text-xl">
            📊
          </div>
          <h3 class="text-xl font-semibold ml-4">Flow Simulator</h3>
        </div>
        <p class="text-gray-600 mb-4">Visualize message flows through RabbitMQ patterns in real-time.</p>
        <div class="text-sm text-indigo-600 font-medium">Visual Learning →</div>
      </div>
    </a>
  </div>
  
  <div class="bg-gray-50 p-6 rounded-lg">
    <h2 class="text-2xl font-semibold mb-4">About RabbitMQ Patterns</h2>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div>
        <h3 class="font-semibold mb-2">Message Queuing</h3>
        <p class="text-gray-600 text-sm">
          Basic pattern where producers send messages to queues, and consumers receive them. 
          Perfect for decoupling services and handling asynchronous processing.
        </p>
      </div>
      <div>
        <h3 class="font-semibold mb-2">Work Distribution</h3>
        <p class="text-gray-600 text-sm">
          Multiple workers consume from the same queue, automatically distributing workload. 
          Great for scaling and load balancing heavy tasks.
        </p>
      </div>
      <div>
        <h3 class="font-semibold mb-2">Publish/Subscribe</h3>
        <p class="text-gray-600 text-sm">
          One message broadcasted to multiple consumers via exchanges. 
          Ideal for notifications, events, and real-time updates.
        </p>
      </div>
      <div>
        <h3 class="font-semibold mb-2">Request/Reply</h3>
        <p class="text-gray-600 text-sm">
          Synchronous-style communication over asynchronous messaging. 
          Enables RPC patterns while maintaining loose coupling.
        </p>
      </div>
    </div>
  </div>
</div>