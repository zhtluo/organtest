ssh -t arch@$1 'bash -ls' < script/aws.sh
scp -r arch@$1:./organtest/target/criterion ./criterion
