<script>
  export let ws;
  export let connected;

  let playerName = '';
  let gameStarted = false;
  let myScore = 0;
  let leaderboard = {};
  let winner = null;
  let gameOver = false;

  function handleWebSocketMessage(event) {
    const data = event.detail;
    if (data.demo_type === 'game') {
      if (data.data.type === 'score_update') {
        leaderboard[data.data.player] = data.data.score;
        leaderboard = {...leaderboard};
        
        if (data.data.player === playerName) {
          myScore = data.data.score;
        }
      } else if (data.data.type === 'winner') {
        winner = data.data.player;
        gameOver = true;
      }
    }
  }

  window.addEventListener('websocket-message', handleWebSocketMessage);

  function startGame() {
    if (!playerName.trim()) {
      alert('Please enter your name first!');
      return;
    }
    gameStarted = true;
    fetchCurrentScores();
  }

  async function fetchCurrentScores() {
    try {
      const response = await fetch('http://localhost:3030/api/game/scores');
      if (response.ok) {
        const data = await response.json();
        leaderboard = data.scores;
        if (leaderboard[playerName]) {
          myScore = leaderboard[playerName];
        }
      }
    } catch (error) {
      console.error('Error fetching scores:', error);
    }
  }

  async function handleClick() {
    if (!gameStarted || gameOver) return;

    try {
      const response = await fetch('http://localhost:3030/api/game/click', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ player_name: playerName }),
      });

      if (response.ok) {
        const data = await response.json();
        myScore = data.score;
      }
    } catch (error) {
      console.error('Error handling click:', error);
    }
  }

  function resetGame() {
    gameStarted = false;
    gameOver = false;
    winner = null;
    myScore = 0;
    leaderboard = {};
    playerName = '';
  }

  function handleKeyPress(event) {
    if (event.key === 'Enter') {
      event.preventDefault();
      startGame();
    }
  }

  $: sortedLeaderboard = Object.entries(leaderboard)
    .sort(([,a], [,b]) => b - a)
    .slice(0, 10);
</script>

<div class="space-y-3">
  <div class="bg-purple-50 p-4 rounded-lg">
    <h3 class="text-lg font-semibold text-purple-800 mb-2">Race to 100 Game Demo</h3>
    <p class="text-purple-700">
      This demo shows RabbitMQ fanout exchanges (pub/sub). Score updates are broadcast to all connected clients in real-time. First to 100 wins!
    </p>
  </div>

  {#if !gameStarted}
    <div class="text-center space-y-3">
      <div class="max-w-md mx-auto">
        <label for="player-name" class="block text-sm font-medium text-neutral-700 mb-2">
          Enter your player name:
        </label>
        <input
          id="player-name"
          type="text"
          bind:value={playerName}
          on:keypress={handleKeyPress}
          placeholder="Your name..."
          class="w-full px-3 py-1 border border-neutral-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
        />
      </div>
      <button
        on:click={startGame}
        disabled={!playerName.trim() || !connected}
        class="px-8 py-3 bg-purple-600 text-white rounded-md hover:bg-purple-700 disabled:bg-neutral-400 disabled:cursor-not-allowed text-lg font-semibold"
      >
        Start Game
      </button>
      {#if !connected}
        <p class="text-red-600 text-sm">WebSocket not connected. Please wait...</p>
      {/if}
    </div>
  {:else}
    <div class="grid md:grid-cols-2 gap-6">
      <!-- Game Area -->
      <div class="space-y-3">
        <div class="text-center">
          <h4 class="text-2xl font-bold text-purple-800 mb-2">
            {playerName}'s Score: {myScore}
          </h4>
          <div class="w-full bg-purple-200 rounded-full h-4 mb-4">
            <div
              class="bg-purple-600 h-4 rounded-full transition-all duration-300"
              style="width: {Math.min(myScore, 100)}%"
            ></div>
          </div>
        </div>

        {#if gameOver}
          <div class="text-center space-y-3">
            {#if winner === playerName}
              <div class="text-4xl">🎉</div>
              <h3 class="text-2xl font-bold text-green-600">Congratulations!</h3>
              <p class="text-lg">You won the race to 100!</p>
            {:else}
              <div class="text-4xl">🏁</div>
              <h3 class="text-2xl font-bold text-blue-600">Game Over!</h3>
              <p class="text-lg">{winner} won the race to 100!</p>
            {/if}
            <button
              on:click={resetGame}
              class="px-3 py-1 bg-purple-600 text-white rounded-md hover:bg-purple-700"
            >
              Play Again
            </button>
          </div>
        {:else}
          <div class="text-center">
            <button
              on:click={handleClick}
              class="w-32 h-32 bg-gradient-to-br from-purple-500 to-purple-700 text-white rounded-full text-2xl font-bold hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all duration-150 active:scale-95 shadow-lg"
            >
              CLICK!
            </button>
            <p class="text-neutral-600 mt-2">Click the button to increase your score!</p>
          </div>
        {/if}
      </div>

      <!-- Leaderboard -->
      <div class="bg-neutral-50 p-4 rounded-lg">
        <h4 class="text-lg font-semibold text-neutral-800 mb-4">Live Leaderboard</h4>
        <div class="space-y-2 max-px-3 py-1 overflow-y-auto">
          {#each sortedLeaderboard as [player, score], index (player)}
            <div class="flex items-center justify-between p-2 bg-white rounded border-l-4 {
              index === 0 ? 'border-yellow-500' : 
              index === 1 ? 'border-neutral-400' : 
              index === 2 ? 'border-orange-500' : 'border-neutral-300'
            } {player === playerName ? 'bg-purple-100' : ''}">
              <div class="flex items-center space-x-2">
                <span class="font-bold text-neutral-600">#{index + 1}</span>
                <span class="font-medium {player === playerName ? 'text-purple-800' : 'text-neutral-800'}">
                  {player}
                  {#if player === playerName}
                    <span class="text-xs text-purple-600">(You)</span>
                  {/if}
                </span>
              </div>
              <div class="flex items-center space-x-2">
                <span class="font-bold text-neutral-800">{score}</span>
                <div class="w-20 bg-neutral-200 rounded-full h-2">
                  <div
                    class="bg-purple-600 h-2 rounded-full transition-all duration-300"
                    style="width: {Math.min(score, 100)}%"
                  ></div>
                </div>
              </div>
            </div>
          {:else}
            <div class="text-center text-neutral-500 py-3">
              No players yet. Start clicking to appear on the leaderboard!
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>