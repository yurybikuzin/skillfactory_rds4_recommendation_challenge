#!/usr/bin/env bash
target_host=bikuzin18
target_dir=/home/bikuzin/recommend

echo Creating $target_host:$target_dir . . .
ssh $target_host "if [[ ! -e $target_dir ]]; then mkdir -p $target_dir; fi"

echo Stopping docker container . . .
ssh $target_host "cd $target_dir && docker-compose down"

echo Copiing prod/.env to $target_host:$target_dir/.env . . .
scp prod/.env $target_host:$target_dir/.env
echo Copiing prod/docker-compose.yml to $target_host:$target_dir/docker-compose.yml . . .
scp prod/docker-compose.yml $target_host:$target_dir/docker-compose.yml

echo Starting docker container . . .
ssh $target_host "cd $target_dir && docker-compose up -d"

echo Copiing prod/nginx.conf to $target_host:$target_dir/nginx.conf . . .
scp prod/nginx.conf $target_host:$target_dir/nginx.conf
echo Copiing prod/patch_main_nginx_conf.bash to $target_host:$target_dir/patch_main_nginx_conf.bash . . .
scp prod/patch_main_nginx_conf.bash $target_host:$target_dir/patch_main_nginx_conf.bash
echo chmod a+x $target_dir/patch_main_nginx_conf.bash . . .
ssh $target_host "chmod a+x $target_dir/patch_main_nginx_conf.bash"

echo Copiing prod/reload_nginx_conf.bash to $target_host:$target_dir/reload_nginx_conf.bash . . .
scp prod/reload_nginx_conf.bash $target_host:$target_dir/reload_nginx_conf.bash
echo chmod a+x $target_dir/reload_nginx_conf.bash . . .
ssh $target_host "chmod a+x $target_dir/reload_nginx_conf.bash"

echo Apply patch to /etc/nginx/main.conf . . .
ssh $target_host "sudo $target_dir/patch_main_nginx_conf.bash"
echo Restart nginx . . .
ssh $target_host "sudo $target_dir/reload_nginx_conf.bash"

echo Done
echo ============ Check result ============
curl -i https://$target_host.baza-winner.ru/amazing

