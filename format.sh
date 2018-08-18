#!/bin/sh

echo 'applying yapf ...'
yapf -r -i -vv .

echo 'applying isort ...'
isort -y

echo 'done'
