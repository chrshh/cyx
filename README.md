# Build                                                                        docker build -t cjyx

# Run normally

  docker run -it cjyx

                                                            # Run as PID 1 (no init process wrapping your shell)                           docker run -it --init=false cjyx
