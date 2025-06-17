<script>
  export let ws;
  export let connected;

  let message = '';
  let messages = [];
  let loading = false;

  function handleWebSocketMessage(event) {
    const data = event.detail;
    if (data.demo_type === 'logger') {
      messages = [...messages, {
        text: data.data.message,
        timestamp: new Date(data.data.timestamp).toLocaleTimeString(),
        id: Date.now()
      }];
    }
  }

  window.addEventListener('websocket-message', handleWebSocketMessage);

  async function sendMessage() {
    if (!message.trim() || loading) return;

    loading = true;
    try {
      const response = await fetch('http://localhost:3030/api/logger/send', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ message: message.trim() }),
      });

      if (response.ok) {
        message = '';
      } else {
        console.error('Failed to send message');
      }
    } catch (error) {
      console.error('Error sending message:', error);
    } finally {
      loading = false;
    }
  }

  function handleKeyPress(event) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }

  function clearMessages() {
    messages = [];
  }
</script>

<div class="space-y-3">
  <div class="flex space-x-3">
    <div class="flex-1">
      <label for="message-input" class="block text-sm font-medium text-neutral-700 mb-2">
        Enter your message:
      </label>
      <textarea
        id="message-input"
        bind:value={message}
        on:keypress={handleKeyPress}
        placeholder="Type your message here..."
        rows="3"
        class="w-full px-3 py-1 border border-neutral-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        disabled={loading}
      ></textarea>
    </div>
  </div>

  <div class="flex space-x-3">
    <button
      on:click={sendMessage}
      disabled={!message.trim() || loading || !connected}
      class="px-3 py-1 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-neutral-400 disabled:cursor-not-allowed flex items-center"
    >
      {#if loading}
        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        Sending...
      {:else}
        Send Message
      {/if}
    </button>

    <button
      on:click={clearMessages}
      class="px-3 py-1 bg-neutral-600 text-white rounded-md hover:bg-neutral-700"
    >
      Clear Messages
    </button>
  </div>

  <div class="bg-neutral-50 p-4 rounded-lg">
    <div class="flex justify-between items-center mb-4">
      <h4 class="text-lg font-semibold text-neutral-800">Messages ({messages.length})</h4>
      <div class="text-sm text-neutral-600">
        Status: {connected ? 'Connected' : 'Disconnected'}
      </div>
    </div>

    <div class="space-y-2 max-px-3 py-1 overflow-y-auto">
      {#each messages as msg (msg.id)}
        <div class="bg-white p-3 rounded border-l-4 border-blue-500">
          <div class="flex justify-between items-start">
            <p class="text-neutral-800 flex-1">{msg.text}</p>
            <span class="text-xs text-neutral-500 ml-4">{msg.timestamp}</span>
          </div>
        </div>
      {:else}
        <div class="text-center text-neutral-500 py-8">
          No messages yet. Send your first message above!
        </div>
      {/each}
    </div>
  </div>
</div>