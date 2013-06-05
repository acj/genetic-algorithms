package ga

import Defs._

object GAWorksheet {
	var targetString = "TheQuickBrownFoxJumpedOverTheLazyDog"
                                                  //> targetString  : String = TheQuickBrownFoxJumpedOverTheLazyDog
	var targetFitness = targetString.length   //> targetFitness  : Int = 36
	
	var popSize = 10                          //> popSize  : Int = 10
	var chromosomeSize = targetString.length  //> chromosomeSize  : Int = 36
  var pop = Seq.fill(popSize)(Chromosome.generateChromosome(chromosomeSize)).toList
                                                  //> pop  : List[ga.Chromosome] = List(List(x, 9, 3, 9, d, i, 2, g, j, j, b, r, J
                                                  //| , g, R, 4, I, l, P, Y, e, Z, X, 0, P, 7, F, D, 2, o, d, 6, M, z, c, 6), List
                                                  //| (K, j, I, R, C, A, F, e, L, E, x, i, k, 9, 5, W, J, 6, 6, t, f, E, q, 9, x, 
                                                  //| B, I, L, 7, A, m, e, Q, I, b, v), List(R, c, o, 2, h, 3, s, C, F, O, o, C, Z
                                                  //| , L, 4, P, M, O, R, a, 2, V, w, X, q, i, U, z, B, z, Z, n, e, f, J, 7), List
                                                  //| (9, L, H, n, Q, t, 5, 4, u, 9, 9, K, i, n, s, f, W, f, j, o, V, y, s, U, d, 
                                                  //| R, U, b, 3, 8, P, A, C, a, 9, 8), List(A, h, L, H, r, M, x, d, I, u, o, R, X
                                                  //| , 6, n, H, u, s, i, V, c, j, r, r, J, b, F, Y, 3, S, j, M, d, K, y, x), List
                                                  //| (w, P, z, y, 3, v, o, l, V, 2, 8, 6, 5, D, r, z, l, b, h, z, l, 3, Z, 8, T, 
                                                  //| B, V, N, a, 7, q, Q, E, n, 0, P), List(x, j, M, 4, W, 3, n, q, 3, 0, L, f, G
                                                  //| , c, k, n, r, a, x, C, I, S, k, H, w, G, Q, i, M, o, B, Y, z, z, Q, r), List
                                                  //| (q, Z, 8, a, K, s, A, j, G, p, V, h, i, D, i, R, g, q, l, 1, T, M, h, 9, u, 
                                                  //| G, D, J, 4, O, q, 6, 8, O, v, 1), List(L, 0, 8, l, 7, F, P, B, r, M, T, J, D
                                                  //| , 1, Z, H, Z, S, w, b, S, p, C, Q, X, e, i, E, e, a, t, j, e, s, P, s), List
                                                  //| (S, e, H, d, l, B, t, k, y, V, 1, a, W, K, B, s, N, M, G, d, K, F, X, u, 4, 
                                                  //| z, i, n, Q, 4, b, b, w, I, x, r))

  var fitnessFunction = {
  	def countMatchingChars( pairs: List[(Char, Char)] ): Float =
  		pairs map((chars: (Char, Char)) => chars match { case (c1, c2) => if (c1 == c2) 1 else 0 }) sum
  	 
  	((c: Chromosome) => countMatchingChars(c.getGenes zip targetString))
  }                                               //> fitnessFunction  : ga.Chromosome => Float = <function1>
  
  var ga = new GeneticAlgorithm(pop, fitnessFunction, targetFitness)
                                                  //> ga  : ga.GeneticAlgorithm = ga.GeneticAlgorithm@175093f1
  
  ga evolve                                       //> Generation 500
                                                  //| Generation 1000
}