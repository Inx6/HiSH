<template>
    <div class="foots">
        <mu-container>
            <mu-row gutter>
                <mu-col span="2">友情链接:</mu-col>
                <mu-col span="1">
                    <a href="http://inis.cc">inis</a>
                </mu-col>
            </mu-row>
            <mu-row gutter>
                <mu-col span="5" style="margin-top:5px;line-height:5vh">当前为北京时间: {{datas}}</mu-col>
            </mu-row>
        </mu-container>
    </div>
</template>

<script>
export default {
    name: "backlinks",
    data(){
        return{
            datas: ''
        }
    },
    mounted(){
        setInterval(()=>{
            this.datas = this.moment().format('MMMM Do YYYY, h:mm:ss a');
        },1000)
    }
}
</script>

<style>
.foots{
    background-color: #2196f3;
    height: 10vh;
    /* width: 100%; */
    color: aliceblue;
    text-align: center;
    /* line-height: 5vh; */
}
</style>