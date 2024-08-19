<script lang="ts">
  import { Canvas } from '@threlte/core';
  import Scene from './lib/Scene.svelte';
  import Navbar from './lib/Navbar.svelte';
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

  .background-glow {
    position: absolute;
    z-index: -100;

    width: 0px;
    height: 0px;
    
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    
    background-color: rgba(138, 43, 226, 0.9);
    box-shadow: 0 0 50vh 20vh rgba(138, 43, 226, 0.65);
  }

  @media (prefers-color-scheme: light) {
    .background-glow {
      background-color: rgba(138, 43, 226, 0.9);
      box-shadow: 0 0 50vh 20vh rgba(138, 43, 226, 0.65);
    }
  }
</style>
