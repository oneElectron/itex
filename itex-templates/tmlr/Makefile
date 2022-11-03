NAME = main

pdf:
	@pdflatex -no-shell-escape -output-directory ./out/ $(NAME).tex
	@if [ -a ./out/main.aux ]; then rm ./out/$(MAIN).aux; fi;
	@if [ -a ./out/main.out ]; then rm ./out/$(MAIN).aux; fi;
	@if [ -a ./out/main.log ]; then rm ./out/$(MAIN).log; fi;
	@if [ -a ./out/main.bcf ]; then rm ./out/$(MAIN).bcf; fi;
	@if [ -a ./out/main.toc ]; then rm ./out/$(MAIN).toc; fi;
	@if [ -a ./out/texput.log ]; then rm ./out/texput.log; fi;
	@if [ -a ./out/main.run.xml ]; then rm ./out/main.run.xml; fi;

debug:
	pdflatex -no-shell-escape -output-directory ./out/ main.tex

