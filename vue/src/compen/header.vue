<template>
    <div>
        <mu-appbar style="width: 100%;" color="rgb(255 187 212)">
            ShinX
            <mu-button flat slot="right" @click="login" v-if="index1">LOGIN</mu-button>
            <mu-button flat slot="right" v-if="index2">正在考虑开发中...</mu-button>
        </mu-appbar>
    </div>
</template>

<script>
export default {
    name: "headers",
    data(){
        return {
            index1: true,
            index2: false
        }
    },
    mounted(){
        if(window.localStorage.getItem("uuid")){
            this.index1 = false;
            this.index2 = true;
        }else{
            this.index1 = true;
            this.index2 = false;
        }
    },
    methods:{
        login(){
            this.$router.push({name: "login"});
        }
    }
}
</script>

<style scoped>

</style>