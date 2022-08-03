<template>
<!-- after 2h I still don't know how to create components with js. -->
<!-- for now, I have to use this: -->
<table id="EasyBoard" cellspacing="0">
    <tr>
        <CellButton x="0" y="0"></CellButton>
        <CellButton x="1" y="0"></CellButton>
        <CellButton x="2" y="0"></CellButton>
        <CellButton x="3" y="0"></CellButton>
        <CellButton x="4" y="0"></CellButton>
        <CellButton x="5" y="0"></CellButton>
        <CellButton x="6" y="0"></CellButton>
        <CellButton x="7" y="0"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="1"></CellButton>
        <CellButton x="1" y="1"></CellButton>
        <CellButton x="2" y="1"></CellButton>
        <CellButton x="3" y="1"></CellButton>
        <CellButton x="4" y="1"></CellButton>
        <CellButton x="5" y="1"></CellButton>
        <CellButton x="6" y="1"></CellButton>
        <CellButton x="7" y="1"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="2"></CellButton>
        <CellButton x="1" y="2"></CellButton>
        <CellButton x="2" y="2"></CellButton>
        <CellButton x="3" y="2"></CellButton>
        <CellButton x="4" y="2"></CellButton>
        <CellButton x="5" y="2"></CellButton>
        <CellButton x="6" y="2"></CellButton>
        <CellButton x="7" y="2"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="3"></CellButton>
        <CellButton x="1" y="3"></CellButton>
        <CellButton x="2" y="3"></CellButton>
        <CellButton x="3" y="3"></CellButton>
        <CellButton x="4" y="3"></CellButton>
        <CellButton x="5" y="3"></CellButton>
        <CellButton x="6" y="3"></CellButton>
        <CellButton x="7" y="3"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="4"></CellButton>
        <CellButton x="1" y="4"></CellButton>
        <CellButton x="2" y="4"></CellButton>
        <CellButton x="3" y="4"></CellButton>
        <CellButton x="4" y="4"></CellButton>
        <CellButton x="5" y="4"></CellButton>
        <CellButton x="6" y="4"></CellButton>
        <CellButton x="7" y="4"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="5"></CellButton>
        <CellButton x="1" y="5"></CellButton>
        <CellButton x="2" y="5"></CellButton>
        <CellButton x="3" y="5"></CellButton>
        <CellButton x="4" y="5"></CellButton>
        <CellButton x="5" y="5"></CellButton>
        <CellButton x="6" y="5"></CellButton>
        <CellButton x="7" y="5"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="6"></CellButton>
        <CellButton x="1" y="6"></CellButton>
        <CellButton x="2" y="6"></CellButton>
        <CellButton x="3" y="6"></CellButton>
        <CellButton x="4" y="6"></CellButton>
        <CellButton x="5" y="6"></CellButton>
        <CellButton x="6" y="6"></CellButton>
        <CellButton x="7" y="6"></CellButton>
    </tr>
    <tr>
        <CellButton x="0" y="7"></CellButton>
        <CellButton x="1" y="7"></CellButton>
        <CellButton x="2" y="7"></CellButton>
        <CellButton x="3" y="7"></CellButton>
        <CellButton x="4" y="7"></CellButton>
        <CellButton x="5" y="7"></CellButton>
        <CellButton x="6" y="7"></CellButton>
        <CellButton x="7" y="7"></CellButton>
    </tr>
</table>
</template>
<script>
import CellButton from './CellButton.vue'

export default {
components: {
    CellButton
        },
    name: "EasyBoard",
    methods: {
        updateAll() {
            //let Buttons = document.getElementsByTagName("CellButton");
            //console.log(Buttons.length);
        }
    },
}
</script>
<style>
#EasyBoard tr {
    border-spacing: 0;
}

</style>
