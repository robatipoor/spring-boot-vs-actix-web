package org.robatipoor.springboottest;

import java.util.Collections;
import java.util.HashSet;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.Collectors;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.*;

@SpringBootApplication
@RestController
public class SpringBootTestApplication {

	private AtomicLong counter = new AtomicLong();
	private Set<Long> products = Collections.newSetFromMap(new ConcurrentHashMap<Long, Boolean>());
	
	@PostMapping("/addProduct")
	public Long addProduct() {
		products.add(counter.incrementAndGet());
		return counter.get();
	}

	@GetMapping("/getProducts")
	public String getProducts() {
		return products.stream().map(String::valueOf).collect(Collectors.joining(" ,"));
	}

	@GetMapping("/count")
	public String getCount() {
		return String.valueOf(products.size());
	}

	public static void main(String[] args) {
		SpringApplication.run(SpringBootTestApplication.class, args);
	}

}


