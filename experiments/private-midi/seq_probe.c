#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
typedef struct { uint8_t type, flags, tag, queue; uint8_t time[8]; uint8_t src_client, src_port, dst_client, dst_port; uint8_t data[16]; } event_t;
int main(int argc, char **argv) {
 if (argc != 3) return 2;
 void *h=dlopen("libasound.so.2", RTLD_NOW); if (!h) return 3;
 int (*open_seq)(void**,const char*,int,int)=dlsym(h,"snd_seq_open");
 int (*close_seq)(void*)=dlsym(h,"snd_seq_close");
 int (*set_name)(void*,const char*)=dlsym(h,"snd_seq_set_client_name");
 int (*info_new)(void**)=dlsym(h,"snd_seq_port_info_malloc");
 int (*get_info)(void*,int,void*)=dlsym(h,"snd_seq_get_port_info");
 int (*read_use)(void*)=dlsym(h,"snd_seq_port_info_get_read_use");
 void (*free_info)(void*)=dlsym(h,"snd_seq_port_info_free");
 int (*create)(void*,const char*,unsigned int,unsigned int)=dlsym(h,"snd_seq_create_simple_port");
 int (*encode_new)(size_t,void**)=dlsym(h,"snd_midi_event_new");
 long (*encode)(void*,const unsigned char*,long,event_t*)=dlsym(h,"snd_midi_event_encode");
 int (*out)(void*,event_t*)=dlsym(h,"snd_seq_event_output_direct");
 void (*enc_free)(void*)=dlsym(h,"snd_midi_event_free");
 if(!open_seq||!close_seq||!create||!encode_new||!encode||!out||!enc_free) return 4;
 void *seq=0,*coder=0; int r=open_seq(&seq,"default",1,0); if(r<0){printf("open %d\n",r);return 5;}
 set_name(seq,"ArpeggiGator Private Test");
 int port=create(seq,"Internal ArpeggiGator",1|32,2|1048576);
 void *info=0; if(info_new(&info)<0)return 8;
 int ready=0; for(int i=0;i<150;i++){ if(get_info(seq,port,info)>=0 && read_use(info)>0){ready=1;break;} usleep(100000); }
 free_info(info); if(!ready){printf("no subscriber\n");return 9;} printf("subscriber ready\n"); fflush(stdout); sleep(2); if(port<0){printf("create %d\n",port);return 6;}
 if(encode_new(32,&coder)<0)return 7;
 int dest=atoi(argv[1]), note=atoi(argv[2]);
 for(int step=0;step<8;step++) { int on=(step%2)==0; event_t e; memset(&e,0,sizeof e); unsigned char msg[3]={(unsigned char)0x90,(unsigned char)(on?note:note-1),(unsigned char)110};
  long n=encode(coder,msg,3,&e); e.queue=253; e.src_port=port; e.dst_client=254; e.dst_port=253;
  r=out(seq,&e); printf("%s encode=%ld result=%d\n",on?"on":"off",n,r); fflush(stdout); usleep(125000);
 }
 for(int i=0;i<2;i++){ event_t e; memset(&e,0,sizeof e); unsigned char msg[3]={0x80,(unsigned char)(note-i),0}; encode(coder,msg,3,&e); e.queue=253;e.src_port=port;e.dst_client=254;e.dst_port=253;out(seq,&e); }
 enc_free(coder);close_seq(seq);return 0;
}
