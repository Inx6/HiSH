<template>
  <div id="app" :style="{backgroundImage: `url(${url})`}" v-if="pc">
    <keep-Alive>
      <router-view v-if="this.$route.meta.keepAlive"/>
    </keep-Alive>
    <router-view v-if="!this.$route.meta.keepAlive"/>
  </div>
</template>

<script>
export default {
  name: 'App',
  data(){
    return{
       pc: true,
       url: require("C:/Users/HongTJiang/Pictures/Saved Pictures/bg.gif")
    }
  },
  mounted(){
    if (this._isMobile()) {
      this.pc = false
      alert("暂未开放");
      window.location.href="http://inis.cc";
    }
  },
  methods:{
    _isMobile() {
      let flag = navigator.userAgent.match(/(phone|pad|pod|iPhone|iPod|ios|iPad|Android|Mobile|BlackBerry|IEMobile|MQQBrowser|JUC|Fennec|wOSBrowser|BrowserNG|WebOS|Symbian|Windows Phone)/i)
      return flag;
    }
  }
}
</script>

<style>
*{
  margin: "0";
  padding: "0";
}
#app{
  /* background-repeat: no-repeat; */
  background-size: auto;
  /* font-family: "Open Sans", cursive, serif, sans-serif; */
  font-size: 16px;
}
</style>
