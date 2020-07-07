#!/bin/bash
starttime=`date +'%Y-%m-%d %H:%M:%S'`
for((i=0;i<4;i++));do
{
    index=1
    while(($index<100000))
    do 
        touch /tmp/${i}fanotify_test
        echo $index > /tmp/${i}fanotify_test
        cat /tmp/${i}fanotify_test
        rm /tmp/${i}fanotify_test
        let "index++"
    done
}&
done
wait
endtime=`date +'%Y-%m-%d %H:%M:%S'`
start_seconds=$(date --date="$starttime" +%s);
end_seconds=$(date --date="$endtime" +%s);
secs=`expr $end_seconds - $start_seconds`
val=`expr 1200000 / $secs`
echo Test QPS:$val