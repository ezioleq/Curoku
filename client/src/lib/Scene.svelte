<script lang="ts">
  import { extend, T, useTask, useThrelte } from '@threlte/core';
  import { type Mesh } from 'three';
  import { OrbitControls } from 'three/examples/jsm/Addons.js';

  extend({OrbitControls})  

  let cube: Mesh;
  
  const {camera, renderer} = useThrelte();
  camera.current.position.z = 1.5

  useTask((delta) => {})
</script>

<group>
  <T.Mesh bind={cube}>
    <T.BoxGeometry args={[1, 1, 1]}/>
    <T.MeshBasicMaterial color="white"/>
  </T.Mesh>

  <T.OrbitControls args={[$camera, renderer.domElement]}/>
</group>
