#定义一些函数, 便捷测试

#一些初始参数
project_name=$(basename $(pwd))
project_path=$(pwd)

#定义myrust函数,方便对test目录下单个文件进行编译运行
function myrust(){
    # 如果目标目录不存在，则创建它
    if [ ! -d "./src/targets" ]; then
        mkdir -p ./src/targets
    fi

    # 检查是否提供了文件名参数
    if [ -z "$1" ]; then
        echo "已刷新当前项目单个二进制文件别名, 如果要编译某个文件, Please provide a filename (without .rs extension)."
        #echo "Usage: myrust <filename>"
    else
        FileName=$1
        FilePath="./src/test/${FileName}.rs"
        TargetPath="./src/targets/${FileName}"
        rustc ${FilePath} -o ${TargetPath}
    fi
    
    # 获取targets目录下的所有可执行文件名
    mytargets=($(ls ./src/targets))
    for target in "${mytargets[@]}"; do
        alias "${target}"="./src/targets/${target}"
        echo "Alias created: ${target} -> ./src/targets/${target}"
    done
}

#定义run函数, 便捷运行当前项目可执行文件
function run() {
    echo -e "\x1b[38;2;128128;164m执行测试初始工作区项目...\x1b[0m"
  #获取当前项目名称
  ${project_path}/target/debug/${project_name}
}
#再申请个别名
alias t='run'