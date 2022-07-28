<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js App"/>
  <Board msg="Board :thumbsup:"/>
  <button @click="test">count is {{count}}</button>
  <button @click="reveal">reveal: {{count}}</button>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import { invoke } from "@tauri-apps/api/tauri"

export default {
  name: 'App',
  components: {
    HelloWorld
  },
  data() {
    return {
        count: "test"
    }
  },
  methods: {
        test: function test(){invoke()},
        reveal: function reveal(){
            invoke('reveal', {x: 7, y:7}).then((message) => {
                        this.count = message
                    })
        },
        mark: function mark(){
            invoke('mark');
        }
  },
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
