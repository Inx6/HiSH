<template>
  <div>
    <v-headers></v-headers>
    <mu-container>
      <mu-row gutter>
        <mu-col span="3">
          <div>
            <v-mylist></v-mylist>
          </div>
          <mu-row >
            <mu-col span="12" style="text-align:center">
              <!-- 广告栏 -->
              <!-- <v-ad></v-ad> -->
            </mu-col>
          </mu-row>
        </mu-col>
        <mu-col span="9">
            <mu-paper :z-depth="5" style="margin:0.5em;">
              <v-scrollimg :current="content"></v-scrollimg>
            </mu-paper>
            <v-card :current="content"></v-card>
            <v-footers @page="change_page"></v-footers>
        </mu-col>
      </mu-row>
    </mu-container>
    <v-backlink></v-backlink>
  </div>
</template>

<script>
import headers from '../compen/header.vue';
import scrollimg from '../compen/scroll-img.vue';
import mylist from '../compen/list-my.vue';
import cards from '../compen/card.vue';
import footerse from '../compen/footers.vue';
import backlinks from '../compen/footer-list.vue';
import interad from '../compen/interad.vue';

export default {
  name: 'home',
  components:{
    'v-headers': headers,
    'v-scrollimg' : scrollimg,
    'v-mylist' : mylist,
    'v-card': cards,
    'v-footers': footerse,
    'v-backlink': backlinks,
    'v-ad': interad
  },
  data () {
    return {
      show4: true,
      content: [],
      pages: 1
    }
  },
  mounted(){
    this.axios({
      method: "GET",
      url: this.urls1 + "home",
      params: {
        data: "hello"
      }
    }).then((res)=>{
      // console.log(res);
      if (res.status == 200){
        console.log(JSON.stringify(res.data.message));
      }
    });

    this.axios({
      method: "GET",
      url: this.urls2 + "home",
      params: {
        current: this.pages
      }
    }).then((res)=>{
      if (res.status == 200){
        res.data.Datas.forEach(e => {
          this.axios({
            method: "GET",
            url: this.urls2 + "user",
            params: {
              id: "'"+e.uuid+"'"
            }
          }).then((s)=>{
            e.name = s.data.Detus[0].name;
            e.avatar = s.data.Detus[0].avatar;
            this.content.push(e);
          })
        });
      }
    });
  },
  methods:{
    change_page(e){
      if(this.pages !== e){
        this.pages = e;
        this.axios({
          method: "GET",
          url: this.urls2 + "home",
          params:{
            current: e
          }
        }).then((res)=>{
          if (res.status == 200 && res.data.Datas.length > 0){
            this.content = [];
            res.data.Datas.forEach(e => {
              this.axios({
                method: "GET",
                url: this.urls2 + "user",
                params: {
                  id: "'"+e.uuid+"'"
                }
              }).then((s)=>{
                e.name = s.data.Detus[0].name;
                e.avatar = s.data.Detus[0].avatar;
                this.content.push(e);
              })
              // console.log(this.content);
            });
          }
        });
      }else{
        this.$toast.info('请不要重复点击！');
      }
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>
