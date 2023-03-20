NAME = main

pdf:
	@pdflatex -no-shell-escape -output-directory ./out/ $(NAME).tex
	@if [ -a ./out/main.aux ]; then rm ./out/$(NAME).aux; fi;
	@if [ -a ./out/main.out ]; then rm ./out/$(NAME).out; fi;
	@if [ -a ./out/main.log ]; then rm ./out/$(NAME).log; fi;
	@if [ -a ./out/main.bcf ]; then rm ./out/$(NAME).bcf; fi;
	@if [ -a ./out/main.toc ]; then rm ./out/$(NAME).toc; fi;
	@if [ -a ./out/texput.log ]; then rm ./out/texput.log; fi;
	@if [ -a ./out/main.run.xml ]; then rm ./out/main.run.xml; fi;

count:
	@texcount $(NAME).tex

clean: 
	@if [ -a ./out/main.aux ]; then rm ./out/$(NAME).aux; fi;
	@if [ -a ./out/main.out ]; then rm ./out/$(NAME).out; fi;
	@if [ -a ./out/main.log ]; then rm ./out/$(NAME).log; fi;
	@if [ -a ./out/main.bcf ]; then rm ./out/$(NAME).bcf; fi;
	@if [ -a ./out/main.toc ]; then rm ./out/$(NAME).toc; fi;
	@if [ -a ./out/texput.log ]; then rm ./out/texput.log; fi;
	@if [ -a ./out/main.run.xml ]; then rm ./out/main.run.xml; fi;
	
debug:
	pdflatex -no-shell-escape -output-directory ./out/ main.tex
