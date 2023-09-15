import subprocess
import time
def main():
# 启动服务器
    server_process = subprocess.Popen("./target/debug/server", shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    time.sleep(1)
# 启动客户端
    client_process = subprocess.Popen("./target/debug/client",shell=True,stdin=subprocess.PIPE,stdout=subprocess.PIPE)
    time.sleep(1)

# 发送 "Set" 到客户端
    val = "\"test1\""
    print("client do Set test1 test1")
    client_process.stdin.write(b"Set test1 test1\n")
    client_process.stdin.flush()
    time.sleep(0.3)
    # 获取服务端返回的值
    x = client_process.stdout.readline().decode().strip()
    print("Received:", x)

    # 关闭客户端
    client_process.stdin.close()
    #client_process.wait()

    # 关闭服务器
    server_process.stdin.close()
    #server_process.wait()

    # 重启服务器
    server_process = subprocess.Popen("./target/debug/server",shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    time.sleep(0.3)
    # 重启客户端
    client_process = subprocess.Popen("./target/debug/client",shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    time.sleep(0.3)
    # 发送 "Get" 到客户端
    client_process.stdin.write(b"Get test1\n")
    client_process.stdin.flush()
    time.sleep(0.3)
    # 获取客户端返回的值
    time.sleep(0.3)
    output = client_process.stdout.readline().decode().strip()
    print("Output:", output)
    time.sleep(0.3)
    # 检查输出是否与前一个值一致
    if output == val:
        print("Output matches the previous value.")
    else:
        print("Output does not match the previous value!")

    # 关闭服务器和客户端
    server_process.stdin.close()
    client_process.stdin.close()
    
    quit()

if __name__ == "__main__":
    main()
