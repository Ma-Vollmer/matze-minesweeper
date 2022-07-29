<template>
<button @click="reveal">returned: {{x}} </button>
</template>
<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
    props: {
        x: String,
        y: String
    },
    methods: {
        reveal: function reveal() {
                    invoke('reveal', {x: parseInt(this.$props.x), y: parseInt(this.$props.y)})
        }
    }
}
</script>

