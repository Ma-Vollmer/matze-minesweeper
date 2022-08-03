<template>
<img :src="image" alt="" @click="update" id="button"/>
</template>
<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
    name: "CellButton",
    props: {
        x: String,
        y: String
    },
    data() {
        return {
            image: require('../assets/unrevealed.png')
        }
    },
    methods: {
        reveal() {
            invoke('reveal', {x: 5, y: 5}).then((message)=>{console.log(message)})
        },
        update() {
            console.log("in update!");
            invoke('get_info', {x: parseInt(this.x), y: parseInt(this.y)}).then((ret)=>{
                console.log(ret);
                    })
            invoke('reveal', {x: parseInt(this.x), y: parseInt(this.y)}).then((message)=>{console.log(message)})
            invoke('get_info', {x: parseInt(this.x), y: parseInt(this.y)}).then((ret)=>{
                    switch(ret){
                        case "unrevealed": this.image = require('../assets/unrevealed.png'); break;
                        case "bomb": this.image = require('../assets/bomb.png'); break;
                        case "marked": this.image = require('../assets/marked.png'); break;
                        default: this.image = require('../assets/number_' + ret + '.png');
                    }
                    })
        }
    }
}
</script>
<style>
img {
    max-height: 50px;
    max-width: 50px;
}
</style>
