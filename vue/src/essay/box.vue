<template>
    <div>
        <mu-container style="background-color:white">
            <mu-row gutter>
                <mu-col :span="1"></mu-col>
                <mu-col :span="11">
                    <mu-flex justify-content="center">
                        <h1>{{title}}</h1>
                    </mu-flex>
                </mu-col>
                <mu-col :span="1"></mu-col>
                <mu-col :span="7">
                    <div class="bod">
                        {{border}}
                    </div>
                </mu-col>
                <mu-col :span="4"></mu-col>
            </mu-row>
        </mu-container>
    </div>
</template>

<script>
export default {
    name: 'box-w',
    props:{
        msg: String,
        border: String,
        title: String
    }
}
</script>