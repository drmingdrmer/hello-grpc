package main

import (
	"context"
	fmt "fmt"
	"net"

	hd "github.com/drmingdrmer/hello-grpc/grpc-go/helloworld"
	grpc "google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
)

type server struct {
}

func (s *server) SayHello(c context.Context, r *hd.HelloRequest) (*hd.HelloReply, error) {
	fmt.Println("{1}", r.Name)
	return &hd.HelloReply{Message: "good"}, nil
}

func main() {
	port := ":3334"
	lis, err := net.Listen("tcp", port)
	if err != nil {
		fmt.Println("err:", err)
		return
	}

	s := grpc.NewServer()
	hd.RegisterGreeterServer(s, &server{})
	reflection.Register(s)
	s.Serve(lis)

}
