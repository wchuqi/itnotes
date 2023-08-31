# git下载问题

## 现象1
描述：
不知道是不是vpn的原因，`git clone https.git`的时候，一直报错：

```shell
@LAPTOP-F50UTFJI MINGW64 /d/code/github
$ git clone https://github.com/git
Cloning into 'itnotes'...
fatal: unable to access 'https://github.com.git/': Failed to connect to github.com port 443 after 21051 ms: Timed out
```

怀疑是`hosts`配置的原因，排查了不是。

遂改用ssh的方式

```shell
$ ssh-keygen -t rsa -C "mail.com"

@LAPTOP-F50UTFJI MINGW64 xxxxxxxx
$ git clone git@github.com:xxx.git
Cloning into 'xxx'...
The authenticity of host 'github.com (20.205.243.166)' can't be established.
ED25519 key fingerprint is SHA256:+DiY3wvvV6TuJJhbpZisF/zxxxxxxxxxx.
This key is not known by any other names
Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added 'github.com' (ED25519) to the list of known hosts.
remote: Enumerating objects: 3, done.
remote: Counting objects: 100% (3/3), done.
remote: Total 3 (delta 0), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (3/3), done.
```

可以下载了。

再次尝试`git clone https`的方式：

```shell
xxxx@LAPTOP-F50UTFJI MINGW64 xxxx
$ git clone https://github.com/xxx.git
Cloning into 'itnotes'...
remote: Enumerating objects: 3, done.
remote: Counting objects: 100% (3/3), done.
remote: Total 3 (delta 0), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (3/3), done.
```

分析：

应该是ssh的方式时，`Warning: Permanently added 'github.com' (ED25519) to the list of known hosts.`起了作用。

这个时候，`github.com`网站的ssh公钥信息就写在了用户`known_hosts`文件里（linux系统是hosts文件）。

## 现象2
fatal: unable to access ‘https:xxxx’ OpenSSL SSL_read: Connection was reset, errno 10054

报错有open_SSL，这是一个https的加密协议。在未登录的情况下，默认不能使用https协议下载github上面的文件，否则会被协议拦截。
当替换成http协议属于未加密，不会拦截下载请求。

把https换成git可以下载，应该是github内部自定义的通信协议，所以git协议也不会拦截下载请求。
先输入命令解除ssl验证后，`git config --global http.sslVerify false`，再次克隆https协议便不会拦截。

如果是登录情况下，https协议则不会拦截，也可以下载文件。
登录情况指的是配置Git的账号密码，该账号密码就是你的github账号密码，大家通常出现下载报错：fatal: unable to access ‘https:xxxx’ OpenSSL SSL_read: Connection was reset, errno 10054，一般是由于账号没有配置。

总结：
登录账号能解决问题，原因是https协议属于加密协议，需要验证，否则不让各种git操作。
把https替换成git可行是github内部自定义协议，把https替换成http可行是因为http协议属于未加密，否则需要输入命令先解除ssl验证。

```shell
# 把https换成git，可以下载
git clone git://github.com/xxxxxxxxxxx
 
# 把https替换成http协议属于未加密，可以下载
git clone http://github.com/xxxxxxxxxxx
 
# 先输入命令解除ssl验证后，再次克隆https协议，可以下载
git config --global http.sslVerify false
git clone https://github.com/xxxxxxxxxxx
    
# 登录情况,指的是配置Git的账号密码，该账号密码就是你的github账号密码
```

建议：
使用https协议克隆项目到本地会出现很多麻烦，以后还是使用ssh密钥进行克隆方便。
使用ssh进行克隆后，在git push 和pull都需要手动输入密码，如果要避免这样的麻烦，可以在git bash键入命令$ ssh-keygen -p，在设置密码的时候回车即可。

参考：
https://www.cnblogs.com/blog-cjz/p/16198647.html