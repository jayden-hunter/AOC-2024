import re,parse
(e:=lambda w:print(sum(x*y for x,y in parse.findall(r"mul({:d},{:d})",w))))(l:=open("f").read()+"do()")
e(re.sub(r"don't\(\)[\s\S]*?do\(\)","",l))