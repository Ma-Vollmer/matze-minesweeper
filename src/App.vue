<template>
    <!--  <HelloWorld msg="Welcome to Your Vue.js App"/> -->
    <button @click="initBoardBad">new Game</button>
    <EasyBoard/>
</template>

<script>
//import HelloWorld from './components/HelloWorld.vue'
import EasyBoard from './components/EasyBoard.vue'
import { invoke } from "@tauri-apps/api/tauri"
import Vue from "vue"

export default {
  name: 'App',
  components: {
      EasyBoard
  },
  data() {
    return {
        count: "test"
    }
  },
  methods: {
       initBoardBad() {
            invoke('new_board');
            //EasyBoard.methods.updateAll();
       },
      // init Board is unused, but once I find out how to to what I was trying here, I'll use it again
        initBoard(){
            var compClass = Vue.extend("CellButton");
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
                    var newInstance = new compClass({
                        propsData: {
                            x: x,
                            y: y
                        }
                    });
                    newInstance.$mount(row_data);
                    
                    let row_data = document.createElement('td');
                    row_data.innerHTML = "<CellButton/>";
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
