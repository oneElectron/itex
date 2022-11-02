pdf:
	@pdflatex -no-shell-escape -output-directory ./out/ main.tex
	@if [ -a ./out/main.aux ]; then rm ./out/main.aux; fi;
	@if [ -a ./out/main.log ]; then rm ./out/main.log; fi;
	@if [ -a ./out/main.bcf ]; then rm ./out/main.bcf; fi;
	@if [ -a ./out/main.toc ]; then rm ./out/main.toc; fi;
	@if [ -a ./out/texput.log ]; then rm ./out/texput.log; fi;
	@if [ -a ./out/main.run.xml ]; then rm ./out/main.run.xml; fi;

debug:
	pdflatex -no-shell-escape -output-directory ./out/ main.tex
