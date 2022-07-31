<template>
    <img alt="Vue logo" src="./assets/logo.png">
    <!--  <HelloWorld msg="Welcome to Your Vue.js App"/> -->
    <CellButton x="7" y="7"/>
    <Board msg="Board :thumbsup:"/>
    <button @click="mark">count is {{count}}</button>
    <button @click="initBoard">new Game {{count}}</button>
    <div id="board"></div>
</template>

<script>
//import HelloWorld from './components/HelloWorld.vue'
import CellButton from './components/CellButton.vue'
import { invoke } from "@tauri-apps/api/tauri"

export default {
  name: 'App',
  components: {
      CellButton
  },
  data() {
    return {
        count: "test"
    }
  },
  methods: {
        initBoard: function initBoard(){
            this.count = "yea";
this.count = "please";
            invoke('new_Board');
            let table = document.createElement('table');
            let thead = document.createElement('thead');
            let tbody = document.createElement('tbody');

            table.appendChild(thead);
            table.appendChild(tbody);


            for (let y=0; y<8; y++){
                let row = document.createElement('tr');
                //let heading = document.createElement('td');
                for (let x=0; x<8; x++){
                    let row_data = document.createElement('td');
                    row_data.innerHTML = x;
                    row.appendChild(row_data);
                }
                //heading.innerHTML = y;
                thead.appendChild(row);
            }

            document.getElementById("board").appendChild(table);
           },
        test: function test(){invoke()},
        reveal: function reveal(){
            invoke('reveal', {x: 7, y: 7}).then((message) => {
                        this.count = message
                    })
        },
        mark: function mark(){
            document.getElementById("test").innerHTML='<p>yay</p>'
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
