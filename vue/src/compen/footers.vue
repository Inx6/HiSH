<template>
    <div>
        <mu-container style="margin:1em">
            <mu-flex justify-content="center">
                <mu-pagination  @change="alter" raised circle :total="total" :current.sync="current"></mu-pagination>
            </mu-flex>
        </mu-container>
    </div>
</template>

<script>
export default {
    name:"footers",
    data(){
        return{
            current: 1,
            total: 10
        }
    },
    methods:{
        alter(){
            this.$emit("page",this.current);
        }
    },
    mounted(){
        // console.log(this.total);
        this.axios({
            method: "GET",
            url: this.urls2 + "length"
        }).then((res)=>{
            if (res.status == 200){
                // console.log(Math.ceil(res.data.Dates[0].current / 2), res.data.Dates[0].current);
                this.total = Math.ceil(res.data.Dates[0].current / 2)*10;
                // console.log(this.total)
            }
        });
    }
}
</script>