package test;

import org.scalatest.FunSuite
import org.junit.runner.RunWith
import org.scalatest.junit.JUnitRunner
import ga.Chromosome
import ga.GeneticAlgorithm

@RunWith(classOf[JUnitRunner])
class TweetSetSuite extends FunSuite {
  trait TestConfig {
	val testString1 = "The albatross is massaging the porpoise with cheese"
    val testPopSize1 = 10
    val testChromosomeSize1 = testString1.length
  }

  test("Generate chromosomes with correct size") {
    new TestConfig {
      var pop = Seq.fill(testPopSize1)(Chromosome.generateChromosome(testChromosomeSize1))
      
      
      assert((pop.filter((c: Chromosome) => c.getGenes.length == testChromosomeSize1)).size === testPopSize1)
    }
  }
  
  test("Can instantiate GeneticAlgorithm") {
    new TestConfig {
      var pop = Seq.fill(testPopSize1)(Chromosome.generateChromosome(testChromosomeSize1)).toList
      var fitnessFunction = {
  	    def countMatchingChars( pairs: List[(Char, Char)] ): Float =
  	      pairs map((chars: (Char, Char)) => chars match { case (c1, c2) => if (c1 == c2) 1 else 0 }) sum
  	 
  	    ((c: Chromosome) => countMatchingChars(c.getGenes zip testString1))
      }
      
      
      new GeneticAlgorithm(pop, fitnessFunction, testChromosomeSize1.toFloat)
    }
  }
  
  test("No mutation when probability = 0") {
    new TestConfig {
       var chromosome = Chromosome.generateChromosome(testChromosomeSize1)
       var mutatedChromosome = chromosome.mutate(0.0f)
       
       
       var zipped = mutatedChromosome.getGenes zip chromosome.getGenes
       assert(zipped.forall((chars: (Char, Char)) => chars._1 == chars._2))
    }
  }
  
  test("All mutated when probability = 100") {
    new TestConfig {
      var chromosome = Chromosome.generateChromosome(testChromosomeSize1)
      var mutatedChromosome = chromosome.mutate(100.0f)
      
      
      var zipped = mutatedChromosome.getGenes zip chromosome.getGenes
      assert(zipped.forall((chars: (Char, Char)) => chars._1 != chars._2))
    }
  }
}
