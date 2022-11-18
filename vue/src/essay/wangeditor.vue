<template>
    <div>
         <mu-container style="background-color:white">
            <mu-row gutter>
                <mu-col :span="12" v-if="type==='watch'" style="height:30vh">
                    <!-- <mu-flex justify-content="center"> -->
                        <!-- @blur="onEditorBlur($event)"
                        @focus="onEditorFocus($event)"
                        @change="onEditorChange($event)" -->
                        <quill-editor
                            v-model="content"
                            ref="myQuillEditor"
                            :options="editorOption"
                            @change="onEditorChange($event)"
                        ></quill-editor>
                    <!-- </mu-flex> -->
                </mu-col>
                <mu-col :span="12" v-if="type==='publish'" style="height:120vh">
                        <quill-editor
                            v-model="content"
                            ref="myQuillEditor"
                            :options="editorOption1"
                            @change="onEditorChange($event)"
                            style="height:110vh"
                        ></quill-editor>
                    <!-- </mu-flex> -->
                </mu-col>
                <mu-col :span="12">
                    <mu-flex justify-content="end">
                        <button @click="sends" v-if="type === 'watch'">发送评论</button>
                        <button @click="sends" v-if="type === 'publish'">提交文章</button>
                    </mu-flex>
                </mu-col>
            </mu-row>
         </mu-container>
    </div>
</template>

<script>
import { quillEditor } from "vue-quill-editor"; //调用编辑器
import "quill/dist/quill.core.css";
import "quill/dist/quill.snow.css";
import "quill/dist/quill.bubble.css";

export default {
    name: 'editors',
    props:{
      type: {
        type: String
      }
    },
    components: { 
        quillEditor 
    },
    data(){
        return{
            content: "",
            editorOption: {
                placeholder: "请输入评论",
                modules: {
                    toolbar:[
                        ['bold', 'italic', 'underline', 'strike'],    //加粗，斜体，下划线，删除线
                        ['blockquote', 'code-block'],     //引用，代码块
                
                        [{ 'header': 1 }, { 'header': 2 }],        // 标题，键值对的形式；1、2表示字体大小
                        [{ 'list': 'ordered'}, { 'list': 'bullet' }],     //列表
                        [{ 'script': 'sub'}, { 'script': 'super' }],   // 上下标
                        [{ 'indent': '-1'}, { 'indent': '+1' }],     // 缩进
                        [{ 'direction': 'rtl' }],             // 文本方向
                
                        [{ 'size': ['small', false, 'large', 'huge'] }], // 字体大小
                        [{ 'header': [1, 2, 3, 4, 5, 6, false] }],     //几级标题
                
                        [{ 'color': [] }, { 'background': [] }],     // 字体颜色，字体背景颜色
                        [{ 'font': [] }],     //字体
                        [{ 'align': [] }],    //对齐方式
                
                        ['clean'],    //清除字体样式
                        ['image']    //上传图片'image'、上传视频'video'
                    ], //工具栏设置
                }
            },
            editorOption1: {
                placeholder: "请输入内容",
                modules: {
                    toolbar:[
                        ['bold', 'italic', 'underline', 'strike'],    //加粗，斜体，下划线，删除线
                        ['blockquote', 'code-block'],     //引用，代码块
                
                        [{ 'header': 1 }, { 'header': 2 }],        // 标题，键值对的形式；1、2表示字体大小
                        [{ 'list': 'ordered'}, { 'list': 'bullet' }],     //列表
                        [{ 'script': 'sub'}, { 'script': 'super' }],   // 上下标
                        [{ 'indent': '-1'}, { 'indent': '+1' }],     // 缩进
                        [{ 'direction': 'rtl' }],             // 文本方向
                
                        [{ 'size': ['small', false, 'large', 'huge'] }], // 字体大小
                        [{ 'header': [1, 2, 3, 4, 5, 6, false] }],     //几级标题
                
                        [{ 'color': [] }, { 'background': [] }],     // 字体颜色，字体背景颜色
                        [{ 'font': [] }],     //字体
                        [{ 'align': [] }],    //对齐方式
                
                        ['clean'],    //清除字体样式
                        ['image']    //上传图片'image'、上传视频'video'
                    ], //工具栏设置
                }
            },    
        }
    },
    methods: {
        onEditorReady(editor) { 
            console.log(editor)
        }, // 准备编辑器

        onEditorBlur() {

         }, // 失去焦点事件
        
        onEditorFocus(val, editor) {
            console.log(val); // 富文本获得焦点时的内容
        }, // 获得焦点事件
        
        onEditorChange(e) {
            // console.log(e)
        }, // 内容改变事件

        sends(){
            // console.log(this.content);
            this.loadings().then((e)=>{
              this.content = "";
            })
        },

        loadings(){
          return new Promise((s,j)=>{
            this.$emit("serdes",this.content);
            s(1);
          })
        }
    },
    computed: {
        editor() {
            // false禁止编辑  true语序编辑
            // this.$refs.myQuillEditor.quill.enable(false)
            return this.$refs.myQuillEditor.quill;
        }
    }
}
</script>

<style>
p {
  margin: 10px;
}

.quill-editor {
  height: 20vh;
}

.ql-snow .ql-picker.ql-size .ql-picker-label::before,
.ql-snow .ql-picker.ql-size .ql-picker-item::before {
  content: "14px";
}

.ql-snow .ql-picker.ql-size .ql-picker-label[data-value="small"]::before,
.ql-snow .ql-picker.ql-size .ql-picker-item[data-value="small"]::before {
  content: "10px";
}
.ql-snow .ql-picker.ql-size .ql-picker-label[data-value="large"]::before,
.ql-snow .ql-picker.ql-size .ql-picker-item[data-value="large"]::before {
  content: "18px";
}
.ql-snow .ql-picker.ql-size .ql-picker-label[data-value="huge"]::before,
.ql-snow .ql-picker.ql-size .ql-picker-item[data-value="huge"]::before {
  content: "32px";
}

.ql-snow .ql-picker.ql-header .ql-picker-label::before,
.ql-snow .ql-picker.ql-header .ql-picker-item::before {
  content: "文本";
}
.ql-snow .ql-picker.ql-header .ql-picker-label[data-value="1"]::before,
.ql-snow .ql-picker.ql-header .ql-picker-item[data-value="1"]::before {
  content: "标题1";
}
.ql-snow .ql-picker.ql-header .ql-picker-label[data-value="2"]::before,
.ql-snow .ql-picker.ql-header .ql-picker-item[data-value="2"]::before {
  content: "标题2";
}
.ql-snow .ql-picker.ql-header .ql-picker-label[data-value="3"]::before,
.ql-snow .ql-picker.ql-header .ql-picker-item[data-value="3"]::before {
  content: "标题3";
}
.ql-snow .ql-picker.ql-header .ql-picker-label[data-value="4"]::before,
.ql-snow .ql-picker.ql-header .ql-picker-item[data-value="4"]::before {
  content: "标题4";
}
.ql-snow .ql-picker.ql-header .ql-picker-label[data-value="5"]::before,
.ql-snow .ql-picker.ql-header .ql-picker-item[data-value="5"]::before {
  content: "标题5";
}
.ql-snow .ql-picker.ql-header .ql-picker-label[data-value="6"]::before,
.ql-snow .ql-picker.ql-header .ql-picker-item[data-value="6"]::before {
  content: "标题6";
}

.ql-snow .ql-picker.ql-font .ql-picker-label::before,
.ql-snow .ql-picker.ql-font .ql-picker-item::before {
  content: "标准字体";
}
.ql-snow .ql-picker.ql-font .ql-picker-label[data-value="serif"]::before,
.ql-snow .ql-picker.ql-font .ql-picker-item[data-value="serif"]::before {
  content: "衬线字体";
}
.ql-snow .ql-picker.ql-font .ql-picker-label[data-value="monospace"]::before,
.ql-snow .ql-picker.ql-font .ql-picker-item[data-value="monospace"]::before {
  content: "等宽字体";
}
</style>