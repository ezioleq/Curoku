<script lang="ts">
  import { Canvas } from '@threlte/core';
  import Scene from './lib/Scene.svelte';
</script>

<main>
  <div class="container">
    <Canvas>
      <Scene/>
    </Canvas>
  </div>
</main>

<style>
  .container {
    height: 100vh;
  }
</style>
