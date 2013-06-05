package ga

import Defs._
import scala.util.Random

class GeneticAlgorithm(pop: Population, fitnessFunction: FitnessFunction, targetFitness: Float) {
  final var MAX_GENERATIONS = 250000
  
  final var RECOM_PROB = 0.80f
  final var MUT_PROB   = 0.15f
  final var REPRO_PROB = 0.05f
  
  final var MUT_PROB_PER_SITE = 0.10f
  
  def evaluate(p: Population): List[Float] = p map fitnessFunction
	
  def mutate(p: Population): Population = {
    p map (
      (c: Chromosome) =>
        Random.nextFloat match {
          case prob if prob < REPRO_PROB => c
          case prob if prob < MUT_PROB => c mutate MUT_PROB_PER_SITE
          case prob => c recombine weightedRandomSelection((p zip evaluate(p)) sortBy(_._2) reverse) 
        }
      )
  }
  
  def select(p: Population): Population = {
    def selectAcc( evalPop: List[(Chromosome, Float)], accPop: Population ): Population = {
      if (accPop.size == evalPop.size) accPop
      else selectAcc(evalPop, weightedRandomSelection(evalPop) :: accPop) 
    }
    
    selectAcc((p zip evaluate(p)) sortBy(_._2) reverse, List[Chromosome]())
  }
  
  def hasOptimalChromosome(p: Population) =
    if (p exists ((c: Chromosome) => fitnessFunction(c) == targetFitness)) true
    else false
  
  def evolve = {
    def evolveAcc(p: Population, gen: Int): Population = {
      if ( gen % 500 == 0 ) println("Generation " + gen)
      mutate(select(p)) match {
        case mutatedPop => 
          if (gen == MAX_GENERATIONS) {
            mutatedPop
          } else if (hasOptimalChromosome(mutatedPop)) {
            println("Found optimal solution")
            println((mutatedPop zip evaluate(mutatedPop)) sortBy(_._2) reverse)
            mutatedPop
          } else evolveAcc(mutatedPop, gen + 1)
      }
    }
    
    evolveAcc(pop, 1)
  }
}