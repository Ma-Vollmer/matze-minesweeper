<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <HelloWorld msg="Welcome to Your Vue.js App"/>
  <button @click="test">count is {{count}}</button>
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
        count: 0
    }
  },
  methods: {
       test: function test(){
           this.count++;
           invoke('new_board')
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
