<template>
<img :src="image" alt="" @click="reveal" id="button"/>
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
            this.image = require('../assets/bomb.png');
            invoke('get_info', {x: parseInt(this.x), y: parseInt(this.y)}).then((ret)=>{
                console.log(ret);
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
