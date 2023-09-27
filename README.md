# webvtt yoinker
yoink webvtt from most streaming sites  
  
currently only supports `webvtt` (mime: `text/vtt`)  

## how to run  
whenever you run this just simply input the remote `index.mpd` as the only argument  
this will attempt to download the index.mpd, parse it and download the webvtts found inside of the mpd

## example
`webvtt "https://link-to-totally-legit-webside.com/index.mpd"`
* make sure the link is inside quotes otherwise it might break when running the command  

this will save all webvtt to an folder called `httpslinktototallylegitwebsidecom` (it simply removes all non alphanumeric chars from the base url)

Done!