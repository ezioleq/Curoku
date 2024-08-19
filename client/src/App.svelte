<script lang="ts">
  import { Canvas } from '@threlte/core';
  import Scene from './lib/Scene.svelte';
</script>

<main class="container">
  <Navbar/>
  <Canvas>
    <Scene/>
  </Canvas>
  <div class="background-glow"/>
</main>

<style>
  .container {
    height: 100vh;
  }
</style>
