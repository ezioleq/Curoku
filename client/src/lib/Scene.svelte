<script lang="ts">
  import { extend, T, useTask, useThrelte } from '@threlte/core';
  import { MOUSE, type Mesh } from 'three';
  import { OrbitControls } from 'three/examples/jsm/Addons.js';

  extend({OrbitControls})  

  let cube: Mesh;
  
  const { camera, renderer } = useThrelte();  

  useTask((delta) => {})
</script>

<group>
  <T.Mesh bind={cube}>
    <T.BoxGeometry args={[1, 1, 1]}/>
    <T.MeshBasicMaterial color="white"/>
  </T.Mesh>

  <T.PerspectiveCamera
    makeDefault
    near={0.01}
    position.z={3}
  >
    <T.OrbitControls args={[$camera, renderer.domElement]}
      minDistance={0.5}
      maxDistance={4}
      zoomToCursor={true}
      mouseButtons={{
        LEFT: null,
        MIDDLE: MOUSE.PAN,
        RIGHT: MOUSE.ROTATE
      }}
    />
  </T.PerspectiveCamera>
</group>
